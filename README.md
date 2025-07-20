# Solana Mint Address Generator

A high-performance Rust application that generates Solana mint addresses ending with specific suffixes for pump.fun and lets.bonk tokens. The generator runs on all CPU cores at maximum utilization and saves addresses to a Supabase database.

## Features

- 🚀 **High Performance**: Utilizes all CPU cores at 100% capacity
- 🎯 **Targeted Generation**: Generates addresses ending with "pump" and "bonk" (case sensitive)
- 💾 **Database Integration**: Saves addresses to Supabase with batch upload support
- 💾 **Local Backup**: Saves addresses to local CSV files
- 🔄 **Batch Processing**: Configurable batch sizes for optimal database performance
- 🛠️ **Cross-Platform**: Works on Windows, Linux, and macOS
- 📊 **Real-time Monitoring**: Progress tracking and performance metrics

## Prerequisites

- **Rust** (latest stable version)
- **Git**
- **Supabase Account** (free tier works)
- **Windows Server** (for production deployment)

## Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/Kvickar/rustminter.git
cd rustminter
```

### 2. Set Up Environment
```bash
# Copy the example environment file
cp .env.example .env

# Edit .env with your Supabase credentials
# SUPABASE_URL=your_supabase_project_url
# SUPABASE_ANON_KEY=your_supabase_anon_key
```

### 3. Set Up Database
Run the SQL script in your Supabase SQL editor:
```sql
-- Copy contents of setup_database.sql
```

### 4. Build and Run
```bash
# Build release version
cargo build --release

# Test with a single address
cargo run --release -- pump --count 1

# Generate 1000 pump addresses
cargo run --release -- pump --count 1000 --batch-size 0 --save-local
```

## Usage

### Command Line Options

```bash
cargo run --release -- [COMMAND] [OPTIONS]

Commands:
  pump    Generate pump.fun addresses (ending with "pump")
  bonk    Generate lets.bonk addresses (ending with "bonk")
  both    Generate both types of addresses

Options:
  -c, --count <COUNT>        Number of addresses to generate [default: 1]
  -b, --batch-size <SIZE>    Batch size for database upload (0 = bulk upload) [default: 25]
  -s, --save-local           Save addresses to local CSV files
  -h, --help                 Print help
```

### Examples

```bash
# Generate 1000 pump addresses with bulk upload
cargo run --release -- pump --count 1000 --batch-size 0 --save-local

# Generate 500 bonk addresses with batch upload
cargo run --release -- bonk --count 500 --batch-size 25 --save-local

# Generate both types (250 each)
cargo run --release -- both --count 250 --batch-size 0 --save-local
```

### Windows Batch Scripts

```bash
# Interactive menu
run.bat

# Performance benchmark
benchmark.bat

# Continuous generation (PUMP only)
auto_start.bat

# Continuous generation (both types)
auto_both.bat

# Manage backup files
manage_backups.bat
```

## Database Setup

### 1. Create Supabase Project
1. Go to [supabase.com](https://supabase.com)
2. Create a new project
3. Note your project URL and anon key

### 2. Run Database Script
Execute this SQL in your Supabase SQL editor:

```sql
-- Create mint_addresses table
CREATE TABLE IF NOT EXISTS mint_addresses (
    id BIGSERIAL PRIMARY KEY,
    mint_address VARCHAR(44) UNIQUE NOT NULL,
    private_key TEXT NOT NULL,
    address_type VARCHAR(10) NOT NULL CHECK (address_type IN ('pump', 'bonk')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enable Row Level Security
ALTER TABLE mint_addresses ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts
CREATE POLICY "Allow anonymous inserts" ON mint_addresses
    FOR INSERT WITH CHECK (true);

-- Create index for faster queries
CREATE INDEX IF NOT EXISTS idx_mint_addresses_type ON mint_addresses(address_type);
CREATE INDEX IF NOT EXISTS idx_mint_addresses_created ON mint_addresses(created_at);
```

### 3. Configure Environment
Update your `.env` file:
```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-anon-key-here
```

## Performance

### Expected Performance (addresses/second):
- **2-core CPU**: ~200K attempts/second
- **4-core CPU**: ~400K attempts/second
- **8-core CPU**: ~800K attempts/second
- **16-core CPU**: ~1.6M attempts/second
- **32-core CPU**: ~3.2M attempts/second

### Optimization Tips:
1. **Use `--batch-size 0`** for maximum throughput on large generations
2. **Enable `--save-local`** for backup and offline access
3. **Run on dedicated hardware** for sustained performance
4. **Use SSD storage** for faster binary loading
5. **Close unnecessary applications** to free up CPU resources

## File Structure

```
rustminter/
├── src/
│   └── main.rs              # Main application logic
├── Cargo.toml               # Rust dependencies
├── .env.example             # Environment template
├── setup_database.sql       # Database setup script
├── run.bat                  # Interactive Windows menu
├── auto_start.bat           # Continuous PUMP generation
├── auto_both.bat            # Continuous both types
├── benchmark.bat            # Performance testing
├── manage_backups.bat       # Backup management
└── README.md               # This file
```

## Output Files

### Database
Addresses are stored in the `mint_addresses` table with:
- `mint_address`: The generated Solana address
- `private_key`: Base58-encoded private key
- `address_type`: "pump" or "bonk"
- `created_at`: Timestamp

### Local Files
When using `--save-local`, addresses are saved to:
- `pump_addresses_YYYYMMDD_HHMMSS.txt`: PUMP addresses
- `bonk_addresses_YYYYMMDD_HHMMSS.txt`: BONK addresses

Format: `mint_address,private_key`

## Troubleshooting

### Common Issues:

1. **Supabase Connection Error**
   - Verify your `.env` file has correct credentials
   - Check internet connection
   - Ensure RLS policies are set up correctly

2. **Compilation Errors**
   ```bash
   rustup update
   cargo clean
   cargo build --release
   ```

3. **Low Performance**
   - Check CPU usage in Task Manager
   - Close unnecessary applications
   - Use `--batch-size 0` for large generations

4. **Database Insert Errors**
   - Run the RLS policy fix: `fix_rls_policy.sql`
   - Check Supabase dashboard for errors
   - Verify table structure matches setup script

### Performance Debugging:
```bash
# Check CPU cores
wmic cpu get NumberOfCores,NumberOfLogicalProcessors

# Monitor process
tasklist /FI "IMAGENAME eq solana-mint-generator.exe"
```

## Security Considerations

### For Production Use:
1. **Use environment variables** for sensitive data
2. **Monitor database usage** in Supabase dashboard
3. **Set up database backups**
4. **Consider rate limiting** for public deployments
5. **Use VPN** for remote server access

### Private Key Security:
- Private keys are stored in base58 format
- Keys are saved locally when using `--save-local`
- Database access should be restricted in production

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions:
1. Check the troubleshooting section
2. Review error messages carefully
3. Test with small batches first
4. Check Supabase dashboard for database issues
5. Open an issue on GitHub

---

**Happy address generating!** 🚀 