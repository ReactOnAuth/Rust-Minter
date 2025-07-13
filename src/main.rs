use anyhow::Result;
use bs58;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[derive(Parser)]
#[command(name = "solana-mint-generator")]
#[command(about = "Generate Solana mint addresses with specific suffixes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate addresses ending with 'pump' for pump.fun tokens
    Pump {
        /// Number of addresses to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Batch size for database uploads (0 = upload all at end)
        #[arg(short, long, default_value = "10")]
        batch_size: u32,
        /// Save to local file as backup
        #[arg(long, default_value = "false")]
        save_local: bool,
    },
    /// Generate addresses ending with 'bonk' for lets.bonk tokens
    Bonk {
        /// Number of addresses to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Batch size for database uploads (0 = upload all at end)
        #[arg(short, long, default_value = "10")]
        batch_size: u32,
        /// Save to local file as backup
        #[arg(long, default_value = "false")]
        save_local: bool,
    },
    /// Generate both pump and bonk addresses
    Both {
        /// Number of addresses to generate for each type
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Batch size for database uploads (0 = upload all at end)
        #[arg(short, long, default_value = "10")]
        batch_size: u32,
        /// Save to local file as backup
        #[arg(long, default_value = "false")]
        save_local: bool,
    },
}

#[derive(Serialize, Deserialize)]
struct AddressRecord {
    pub_key: String,
    private_key: String,
    suffix_type: String,
    created_at: String,
}

#[derive(Serialize, Clone)]
struct SupabaseInsert {
    pub_key: String,
    private_key: String, // Base58 encoded for Solana compatibility
    suffix_type: String,
}

struct SupabaseClient {
    client: Client,
    url: String,
    key: String,
}

impl SupabaseClient {
    fn new() -> Result<Self> {
        let url = env::var("SUPABASE_URL")?;
        let key = env::var("SUPABASE_ANON_KEY")?;
        
        Ok(Self {
            client: Client::new(),
            url,
            key,
        })
    }

    async fn insert_address(&self, record: SupabaseInsert) -> Result<()> {
        let response = self
            .client
            .post(&format!("{}/rest/v1/mint_addresses", self.url))
            .header("apikey", &self.key)
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Content-Type", "application/json")
            .json(&record)
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ Successfully saved address {} to Supabase", record.pub_key);
        } else {
            let error_text = response.text().await?;
            println!("❌ Failed to save address: {}", error_text);
        }

        Ok(())
    }

    async fn insert_addresses_batch(&self, records: Vec<SupabaseInsert>) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let response = self
            .client
            .post(&format!("{}/rest/v1/mint_addresses", self.url))
            .header("apikey", &self.key)
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Content-Type", "application/json")
            .json(&records)
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ Successfully saved {} addresses to Supabase in batch", records.len());
        } else {
            let error_text = response.text().await?;
            println!("❌ Failed to save batch: {}", error_text);
            
            // Fallback: try individual inserts
            println!("🔄 Retrying with individual inserts...");
            for record in records {
                if let Err(e) = self.insert_address(record).await {
                    println!("⚠️  Individual insert failed: {}", e);
                }
            }
        }

        Ok(())
    }
}

struct AddressGenerator {
    supabase: SupabaseClient,
    attempts: Arc<AtomicU64>,
}

impl AddressGenerator {
    fn new(supabase: SupabaseClient) -> Self {
        Self {
            supabase,
            attempts: Arc::new(AtomicU64::new(0)),
        }
    }

    async fn generate_addresses(&self, suffix: &str, count: u32, batch_size: u32, save_local: bool) -> Result<Vec<(Keypair, String)>> {
        let mut results = Vec::new();
        let mut batch_records = Vec::new();
        let start_time = Instant::now();
        
        // Get number of CPU cores
        let num_cores = thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1);
        
        println!("🔍 Generating {} addresses ending with '{}' using {} CPU cores...", count, suffix, num_cores);
        println!("🚀 Running at 100% CPU utilization...");
        
        if batch_size == 0 {
            println!("💾 Upload strategy: Save all addresses at the end");
        } else {
            println!("💾 Upload strategy: Batch upload every {} addresses", batch_size);
        }
        
        // Prepare local file if needed
        let mut local_file = if save_local {
            let filename = format!("{}_addresses_{}.txt", suffix, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
            println!("📁 Saving local backup to: {}", filename);
            Some(OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)?)
        } else {
            None
        };
        
        for i in 0..count {
            let keypair = self.find_address_with_suffix(suffix, num_cores).await?;
            let pub_key = keypair.pubkey().to_string();
            
            println!("✨ Found address {}/{}: {}", i + 1, count, pub_key);
            
            // Prepare database record
            let record = SupabaseInsert {
                pub_key: pub_key.clone(),
                private_key: bs58::encode(keypair.to_bytes()).into_string(),
                suffix_type: suffix.to_string(),
            };
            
            // Save to local file if enabled
            if let Some(ref mut file) = local_file {
                writeln!(file, "{},{},{}", pub_key, bs58::encode(keypair.to_bytes()).into_string(), suffix)?;
            }
            
            batch_records.push(record);
            results.push((keypair, pub_key));
            
            // Handle batch uploads
            if batch_size > 0 && batch_records.len() >= batch_size as usize {
                if let Err(e) = self.supabase.insert_addresses_batch(batch_records.clone()).await {
                    println!("⚠️  Failed to save batch to Supabase: {}", e);
                }
                batch_records.clear();
            }
        }
        
        // Upload any remaining records
        if !batch_records.is_empty() {
            if let Err(e) = self.supabase.insert_addresses_batch(batch_records).await {
                println!("⚠️  Failed to save final batch to Supabase: {}", e);
            }
        }
        
        let elapsed = start_time.elapsed();
        let total_attempts = self.attempts.load(Ordering::Relaxed);
        
        println!("\n📊 Generation complete!");
        println!("⏱️  Total time: {:?}", elapsed);
        println!("🎯 Total attempts: {}", total_attempts);
        println!("📈 Average attempts per address: {:.2}", total_attempts as f64 / count as f64);
        println!("⚡ Performance: {:.2} attempts/second", total_attempts as f64 / elapsed.as_secs_f64());
        
        if save_local {
            println!("📁 Local backup saved successfully");
        }
        
        Ok(results)
    }

    async fn find_address_with_suffix(&self, suffix: &str, num_cores: usize) -> Result<Keypair> {
        let (tx, mut rx) = mpsc::channel::<Keypair>(1);
        let found = Arc::new(AtomicBool::new(false));
        let attempts = self.attempts.clone();
        let suffix_owned = suffix.to_string();
        
        // Spawn worker threads on all CPU cores
        let mut handles = Vec::new();
        for thread_id in 0..num_cores {
            let tx = tx.clone();
            let found = found.clone();
            let attempts = attempts.clone();
            let suffix = suffix_owned.clone();
            
            let handle = tokio::task::spawn_blocking(move || {
                let mut local_attempts = 0u64;
                let mut last_report = Instant::now();
                
                loop {
                    // Check if another thread found the address
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
                    
                    let keypair = Keypair::new();
                    let pubkey = keypair.pubkey();
                    let address = pubkey.to_string();
                    
                    local_attempts += 1;
                    attempts.fetch_add(1, Ordering::Relaxed);
                    
                    // Report progress from thread 0 only every 5 seconds
                    if thread_id == 0 && last_report.elapsed() >= Duration::from_secs(5) {
                        let total_attempts = attempts.load(Ordering::Relaxed);
                        println!("🔄 Total attempts: {} (searching for '{}' on {} cores)", 
                                total_attempts, suffix, num_cores);
                        last_report = Instant::now();
                    }
                    
                    if address.ends_with(&suffix) {
                        // Signal other threads to stop
                        found.store(true, Ordering::Relaxed);
                        
                        println!("🎉 Found matching address after {} local attempts on thread {}!", 
                                local_attempts, thread_id);
                        
                        // Send the result
                        if tx.blocking_send(keypair).is_err() {
                            // Channel was closed, another thread might have found it first
                            break;
                        }
                        break;
                    }
                    
                    // No delay - run at 100% CPU
                }
            });
            
            handles.push(handle);
        }
        
        // Drop the original sender so the channel can close when all workers are done
        drop(tx);
        
        // Wait for the first result
        let result = rx.recv().await.ok_or_else(|| {
            anyhow::anyhow!("All worker threads finished without finding a matching address")
        })?;
        
        // Signal all threads to stop
        found.store(true, Ordering::Relaxed);
        
        // Wait for all threads to complete
        for handle in handles {
            let _ = handle.await;
        }
        
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let cli = Cli::parse();
    
    // Initialize Supabase client
    let supabase = SupabaseClient::new().map_err(|e| {
        anyhow::anyhow!(
            "Failed to initialize Supabase client: {}. Please check your .env file.", 
            e
        )
    })?;
    
    let generator = AddressGenerator::new(supabase);
    
    match cli.command {
        Commands::Pump { count, batch_size, save_local } => {
            generator.generate_addresses("pump", count, batch_size, save_local).await?;
        }
        Commands::Bonk { count, batch_size, save_local } => {
            generator.generate_addresses("bonk", count, batch_size, save_local).await?;
        }
        Commands::Both { count, batch_size, save_local } => {
            println!("🚀 Generating both pump and bonk addresses...\n");
            
            generator.generate_addresses("pump", count, batch_size, save_local).await?;
            println!();
            generator.generate_addresses("bonk", count, batch_size, save_local).await?;
        }
    }
    
    Ok(())
} 