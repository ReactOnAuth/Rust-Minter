# Windows Server Setup Guide

Complete guide for setting up and running the Solana Mint Address Generator on a fresh Windows server.

## Prerequisites

- Fresh Windows Server (2019/2022 recommended)
- Administrator access
- Internet connection
- RDP access for initial setup

## Step 1: Update Windows Server

1. **Open Windows Update**
   - Press `Win + I` → Update & Security → Windows Update
   - Click "Check for updates"
   - Install all available updates and restart if needed

2. **Enable Developer Mode** (Optional but recommended)
   - Press `Win + I` → Update & Security → For developers
   - Select "Developer mode"

## Step 2: Install Git

1. **Download Git**
   - Go to https://git-scm.com/download/win
   - Download the latest version for Windows

2. **Install Git**
   - Run the installer as Administrator
   - Use default settings (recommended)
   - Verify installation: Open Command Prompt and run `git --version`

## Step 3: Install Rust

1. **Download Rust**
   - Go to https://rustup.rs/
   - Download `rustup-init.exe`

2. **Install Rust**
   - Run `rustup-init.exe` as Administrator
   - Choose option `1` (default installation)
   - Wait for installation to complete
   - Restart Command Prompt or PowerShell

3. **Verify Installation**
   ```cmd
   rustc --version
   cargo --version
   ```

4. **Update Rust** (ensure latest version)
   ```cmd
   rustup update
   ```

## Step 4: Configure Windows for High Performance

1. **Set High Performance Power Plan**
   - Open Control Panel → Power Options
   - Select "High performance" or "Ultimate Performance"

2. **Disable Windows Defender Real-time Protection** (for maximum performance)
   - Open Windows Security → Virus & threat protection
   - Manage settings → Turn off Real-time protection
   - **Note**: Only do this if you trust the environment

3. **Increase Virtual Memory** (if needed)
   - Right-click "This PC" → Properties → Advanced system settings
   - Performance → Settings → Advanced → Change
   - Set custom size: Initial = 4096 MB, Maximum = 8192 MB

## Step 5: Clone and Build the Project

1. **Create Project Directory**
   ```cmd
   mkdir C:\Projects
   cd C:\Projects
   ```

2. **Clone Repository**
   ```cmd
   git clone https://github.com/Kvickar/rustminter.git
   cd rustminter
   ```

3. **Build Release Version**
   ```cmd
   cargo build --release
   ```
   - This may take 5-10 minutes on first build
   - Subsequent builds will be faster

## Step 6: Test the Application

1. **Quick Test**
   ```cmd
   cargo run --release -- pump --count 1
   ```

2. **Performance Test**
   ```cmd
   benchmark.bat
   ```

3. **Interactive Menu**
   ```cmd
   run.bat
   ```

## Step 7: Server Optimization

### CPU Optimization
- **Disable CPU throttling**
  ```cmd
  powercfg /setacvalueindex scheme_current sub_processor PERFINCPOLICY 0
  powercfg /setactive scheme_current
  ```

### Network Optimization
- **Disable network throttling**
  ```cmd
  netsh int tcp set global autotuninglevel=disabled
  ```

### Memory Optimization
- **Increase working set**
  ```cmd
  wmic process where name="solana-mint-generator.exe" CALL SetPriority "high priority"
  ```

## Step 8: Production Commands

### High-Performance Generation
```cmd
# Generate 1000 pump addresses with bulk upload
cargo run --release -- pump --count 1000 --batch-size 0 --save-local

# Generate 500 bonk addresses with batch upload
cargo run --release -- bonk --count 500 --batch-size 25 --save-local

# Generate both types
cargo run --release -- both --count 250 --batch-size 0 --save-local
```

### Background Processing
```cmd
# Run in background (PowerShell)
Start-Process -FilePath "cargo" -ArgumentList "run --release -- pump --count 10000 --batch-size 0" -WindowStyle Hidden

# Or use nohup equivalent
nohup cargo run --release -- pump --count 10000 --batch-size 0 &
```

## Step 9: Monitoring and Logs

### Performance Monitoring
```cmd
# Monitor CPU usage
wmic cpu get loadpercentage /value

# Monitor memory usage
wmic computersystem get TotalPhysicalMemory

# Monitor process
tasklist /FI "IMAGENAME eq solana-mint-generator.exe"
```

### Log Management
- Output logs are displayed in console
- Use `--save-local` flag to save addresses to files
- Backup files are created in project directory

## Step 10: Automation Scripts

### Auto-start Script (auto_start.bat)
```batch
@echo off
cd C:\Projects\rustminter
echo Starting Solana Mint Generator...
cargo run --release -- pump --count 5000 --batch-size 0 --save-local
echo Generation complete. Restarting in 10 seconds...
timeout /t 10
goto :start
```

### Scheduled Task Setup
```cmd
# Create scheduled task to run every hour
schtasks /create /tn "SolanaMinter" /tr "C:\Projects\rustminter\auto_start.bat" /sc hourly /mo 1
```

## Performance Expectations

### Expected Performance on Server Hardware:
- **2-core server**: ~200K attempts/second
- **4-core server**: ~400K attempts/second
- **8-core server**: ~800K attempts/second
- **16-core server**: ~1.6M attempts/second
- **32-core server**: ~3.2M attempts/second

### Optimization Tips:
1. **Use SSD storage** for faster binary loading
2. **Ensure adequate cooling** for sustained 100% CPU usage
3. **Use batch-size 0** for maximum throughput on large generations
4. **Close unnecessary services** to free up resources
5. **Use dedicated CPU cores** if possible

## Troubleshooting

### Common Issues:

1. **Rust not found**
   - Restart Command Prompt after installation
   - Check PATH environment variable

2. **Git not found**
   - Verify Git is installed and in PATH
   - Try running from Git Bash

3. **Compilation errors**
   - Ensure latest Rust version: `rustup update`
   - Clear cache: `cargo clean`
   - Rebuild: `cargo build --release`

4. **Supabase connection errors**
   - Check internet connection
   - Verify .env file is present
   - Test connection: `ping supabase.co`

5. **Low performance**
   - Check CPU usage in Task Manager
   - Disable Windows Defender
   - Close unnecessary applications
   - Verify High Performance power plan

### Performance Debugging:
```cmd
# Check CPU cores
wmic cpu get NumberOfCores,NumberOfLogicalProcessors

# Check memory
wmic memorychip get capacity

# Check disk speed
winsat disk -drive c
```

## Security Considerations

### For Production Servers:
1. **Enable Windows Firewall** with specific rules
2. **Use VPN** for remote access
3. **Regular updates** for security patches
4. **Monitor resource usage** for unusual activity
5. **Backup configurations** regularly

### Database Security:
- The .env file contains Supabase credentials
- Monitor database usage in Supabase dashboard
- Set up database backups
- Consider rate limiting for production use

## Maintenance

### Regular Tasks:
1. **Update Rust**: `rustup update`
2. **Update dependencies**: `cargo update`
3. **Clean old builds**: `cargo clean`
4. **Monitor disk space** for backup files
5. **Check database storage** in Supabase

### Backup Strategy:
- Generated addresses are saved locally with `--save-local`
- Database is automatically backed up by Supabase
- Consider copying backup files to external storage

## Support

If you encounter any issues:
1. Check this guide first
2. Review error messages carefully
3. Test with small batches first
4. Monitor system resources
5. Check Supabase dashboard for database issues

---

**Ready for high-performance address generation on Windows Server!** 🚀 