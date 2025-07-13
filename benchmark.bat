@echo off
echo ===============================================
echo Solana Mint Address Generator - Performance Test
echo ===============================================
echo.
echo This will generate 1 pump address to test performance
echo on all available CPU cores at 100%% utilization.
echo.
echo Your system will run at maximum CPU capacity during this test.
echo.
set /p confirm="Continue? (y/n): "
if /i not "%confirm%"=="y" (
    echo Benchmark cancelled.
    pause
    exit /b 0
)

echo.
echo 🚀 Starting performance benchmark...
echo ⏱️  Timing address generation with all CPU cores...
echo.

cargo run --release -- pump --count 1 --batch-size 0

echo.
echo 📊 Benchmark complete!
echo.
echo Tips for maximum performance:
echo - Close unnecessary applications
echo - Ensure good CPU cooling
echo - Run in Release mode (--release flag)
echo - Consider overclocking if system supports it
echo.
pause 