# Solana Mint Address Generator

A high-performance Rust application that generates Solana mint addresses with specific suffixes ("pump" for pump.fun tokens and "bonk" for lets.bonk tokens) and automatically stores them in a Supabase database.

## Features

- 🎯 Generate Solana addresses ending with "pump" or "bonk" (case-sensitive)
- 💾 Automatic Supabase database integration
- 📊 Performance tracking and statistics
- 🔄 Progress logging during generation
- 🛠️ Command-line interface with multiple options
- ⚡ **Multi-core processing at 100% CPU utilization**
- 🚀 Utilizes all available CPU cores for maximum performance
- 🔥 Optimized for high-throughput address generation
- 📦 **Batch upload system** - uploads addresses in configurable batches
- 💾 **Local backup support** - saves addresses to local files
- 🔄 **Flexible upload strategies** - immediate, batch, or bulk upload

## 🚀 Key Improvements

### Major Optimizations Made:

**1. Batch Upload System**
- ✅ Configurable batch sizes (1, 10, 25, or bulk)
- ✅ Dramatically reduced database load
- ✅ Automatic fallback to individual inserts if batch fails
- ✅ Memory-efficient batch processing

**2. Local Backup Support**
- ✅ Timestamped backup files
- ✅ CSV format for easy importing
- ✅ Prevents data loss during database issues
- ✅ Backup management utilities

**3. Flexible Upload Strategies**
- ✅ Immediate upload (batch-size 1)
- ✅ Batch upload (batch-size 10) - **Recommended**
- ✅ Bulk upload (batch-size 0) - **Fastest for large batches**
- ✅ Local-only mode with manual upload later

**4. Enhanced Performance**
- ✅ 100% CPU utilization across all cores
- ✅ Reduced database overhead
- ✅ Better progress reporting
- ✅ Comprehensive performance statistics

## Prerequisites

- Rust 1.70+ installed
- Supabase account and project set up
- Windows (for .exe compilation as per user preference)

## Setup

### 1. Clone and Setup Project

```bash
git clone <repository-url>
cd solana-mint-generator
```

### 2. Install Dependencies

```bash
cargo build --release
```

**Note**: Always use `--release` for production use as it provides significantly better performance.

### 3. Configure Supabase

1. Create a new Supabase project at [supabase.com](https://supabase.com)
2. Go to Settings > API to find your project URL and anon key
3. Copy `.env.example` to `.env`:
   ```bash
   copy .env.example .env
   ```
4. Update `.env` with your Supabase credentials:
   ```
   SUPABASE_URL=https://your-project.supabase.co
   SUPABASE_ANON_KEY=your-anon-key
   ```

### 4. Set up Database

1. Go to your Supabase project dashboard
2. Open the SQL Editor
3. Run the contents of `setup_database.sql` to create the required table

## Usage

### Basic Commands

Generate 1 address ending with "pump":
```bash
cargo run --release -- pump
```

Generate 5 addresses ending with "bonk" with batch uploads:
```bash
cargo run --release -- bonk --count 5 --batch-size 10
```

Generate 3 addresses of each type with local backup:
```bash
cargo run --release -- both --count 3 --save-local
```

Generate 100 addresses with bulk upload (all at end):
```bash
cargo run --release -- pump --count 100 --batch-size 0
```

### Command Line Options

```bash
solana-mint-generator <COMMAND>

Commands:
  pump  Generate addresses ending with 'pump' for pump.fun tokens
  bonk  Generate addresses ending with 'bonk' for lets.bonk tokens
  both  Generate both pump and bonk addresses

Options:
  -c, --count <COUNT>        Number of addresses to generate [default: 1]
  -b, --batch-size <SIZE>    Batch size for uploads (0 = all at end) [default: 10]
      --save-local           Save addresses to local backup file
  -h, --help                 Print help
```

### Upload Strategies

**Immediate Upload** (`--batch-size 1`):
- Uploads each address immediately after generation
- Highest database load but lowest memory usage
- Good for small batches or unreliable connections

**Batch Upload** (`--batch-size 10`):
- Uploads addresses in groups of 10 (configurable)
- Balanced approach - good performance with reasonable database load
- **Recommended for most use cases**

**Bulk Upload** (`--batch-size 0`):
- Generates all addresses first, then uploads everything at once
- Fastest generation, lowest database load
- Best for large batches (100+ addresses)

**Local Backup** (`--save-local`):
- Saves addresses to timestamped local files
- Acts as backup in case of database issues
- Format: `pump_addresses_20240101_120000.txt`
- CSV format: `public_key,base58_private_key,suffix_type`
- Private keys are Solana-compatible base58 format

### Building for Windows

To create a Windows executable:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be created at `target/x86_64-pc-windows-gnu/release/solana-mint-generator.exe`

### Running the Application

Now that the project has been updated, you can run commands normally again:

```bash
cargo run --release -- [commands]
```

Or use the built executable directly:
```bash
.\target\release\solana-mint-generator.exe [commands]
```

### Performance Benchmarking

Run the performance benchmark to test your system's capabilities:

```bash
benchmark.bat
```

This will generate one address using all CPU cores and display performance statistics.

For manual performance testing:
```bash
cargo run --release -- pump --count 1
```

### Backup Management

Manage your local backup files with the backup manager:

```bash
manage_backups.bat
```

Features:
- List all backup files
- View backup contents
- Clean old backups
- Convert formats (CSV to JSON)
- Import guidance

### Maximum Performance Tips

For optimal performance:
- Always use `--release` flag for production builds
- Use the proper command: `cargo run --release -- [commands]`
- Close unnecessary applications to free up CPU resources
- Ensure adequate CPU cooling for sustained 100% utilization
- Consider using a high-performance CPU with many cores
- Run from SSD storage for faster binary loading
- Use appropriate batch sizes for your use case
- Enable local backups for peace of mind

## Database Schema

The application creates a `mint_addresses` table with the following structure:

```sql
CREATE TABLE mint_addresses (
    id SERIAL PRIMARY KEY,
    pub_key TEXT NOT NULL UNIQUE,
    private_key TEXT NOT NULL,
    suffix_type TEXT NOT NULL CHECK (suffix_type IN ('pump', 'bonk')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## Performance Notes

### Generation Performance
- **Multi-core Processing**: Automatically detects and uses all CPU cores for maximum performance
- **100% CPU Utilization**: Runs at full CPU capacity for fastest address generation
- **Parallel Workers**: Each CPU core runs a dedicated worker thread
- Address generation is probabilistic - finding addresses with specific suffixes can take time
- Expected time varies based on suffix length, system performance, and number of CPU cores
- The application includes progress logging every 5 seconds
- Performance statistics include attempts per second and total throughput
- **Significant Performance Boost**: Multi-core processing can be 4-16x faster than single-core

### Database Performance
- **Batch Uploads**: Dramatically reduce database load compared to individual inserts
- **Configurable Batch Size**: Balance between memory usage and database efficiency
- **Bulk Upload Mode**: Fastest for large batches (100+ addresses)
- **Fallback Handling**: Automatic retry with individual inserts if batch fails
- **Local Backup**: Prevents data loss if database is unavailable

### Performance Recommendations
- **Small batches (1-20 addresses)**: Use batch size 5-10
- **Medium batches (20-100 addresses)**: Use batch size 10-25
- **Large batches (100+ addresses)**: Use batch size 0 (bulk upload)
- **Unreliable connections**: Use batch size 1 with `--save-local`
- **Maximum speed**: Use batch size 0 with good database connection

## Security Considerations

- Private keys are **base58-encoded** for Solana compatibility and stored in the database
- Private key format: 88-character base58 string (e.g., `4nTk8tn1djjjjKNWjNs4ANR6ynWX4ooq6nBRy9WGAsraRzofUMw4DUkpaGtJGhLBYyNuYpTZQ7FdGoAtz6u3e4vA`)
- Compatible with Solana CLI, web3.js, and other Solana tools
- Ensure your Supabase project has appropriate RLS policies
- Keep your `.env` file secure and never commit it to version control
- Consider additional encryption for private keys in production use

## Error Handling

The application includes comprehensive error handling for:
- Supabase connection issues
- Invalid environment variables
- Database insertion failures
- Network connectivity problems

## Example Output

### Batch Upload Mode
```
🔍 Generating 5 addresses ending with 'pump' using 8 CPU cores...
🚀 Running at 100% CPU utilization...
💾 Upload strategy: Batch upload every 3 addresses
📁 Saving local backup to: pump_addresses_20240101_120000.txt
🔄 Total attempts: 156847 (searching for 'pump' on 8 cores)
🎉 Found matching address after 23156 local attempts on thread 3!
✨ Found address 1/5: 7XvKquFQXpump
🔄 Total attempts: 284593 (searching for 'pump' on 8 cores)
🎉 Found matching address after 18934 local attempts on thread 5!
✨ Found address 2/5: 9YzMqwRSVpump
🔄 Total attempts: 402847 (searching for 'pump' on 8 cores)
🎉 Found matching address after 31245 local attempts on thread 1!
✨ Found address 3/5: 3PqMnRvTSpump
✅ Successfully saved 3 addresses to Supabase in batch
... (continues for remaining addresses)
✅ Successfully saved 2 addresses to Supabase in batch

📊 Generation complete!
⏱️  Total time: 28.3s
🎯 Total attempts: 1,245,890
📈 Average attempts per address: 249,178.00
⚡ Performance: 44,025.12 attempts/second
📁 Local backup saved successfully
```

### Bulk Upload Mode
```
🔍 Generating 10 addresses ending with 'bonk' using 24 CPU cores...
🚀 Running at 100% CPU utilization...
💾 Upload strategy: Save all addresses at the end
... (generation continues)
✅ Successfully saved 10 addresses to Supabase in batch

📊 Generation complete!
⏱️  Total time: 45.2s
🎯 Total attempts: 2,847,293
📈 Average attempts per address: 284,729.30
⚡ Performance: 62,983.45 attempts/second
```

## Troubleshooting

### Common Issues

1. **Supabase RLS Policy Error** (most common)
   ```
   ❌ Failed to save address: {"code":"42501","message":"new row violates row-level security policy"}
   ```
   
   **Solution A** (Recommended): Update your database policies by running this SQL in Supabase:
   ```sql
   -- Allow anonymous users to insert new addresses
   CREATE POLICY "Allow anonymous insert for mint addresses" ON mint_addresses
       FOR INSERT WITH CHECK (true);
   ```
   
   **Quick Fix**: Run the contents of `fix_rls_policy.sql` in your Supabase SQL Editor
   
   **Solution B** (Alternative): Use service role key instead of anon key:
   1. Go to Supabase → Settings → API
   2. Copy the **service_role** key (not anon key)
   3. Replace `SUPABASE_ANON_KEY` in your `.env` file with the service role key
   
   > **Note**: Service role bypasses RLS and has full database access. Use with caution.

2. **Supabase Connection Error**
   - Check your `.env` file configuration
   - Verify your Supabase URL and API key
   - Ensure your Supabase project is active

3. **Database Table Not Found**
   - Run the SQL script in `setup_database.sql`
   - Check table permissions in Supabase

4. **Build Errors**
   - Ensure Rust 1.70+ is installed
   - Run `cargo clean` and `cargo build` again

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is licensed under the MIT License. 