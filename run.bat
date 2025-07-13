@echo off
echo Solana Mint Address Generator
echo ==============================
echo.
echo Available commands:
echo 1. Generate pump addresses
echo 2. Generate bonk addresses  
echo 3. Generate both types
echo.
set /p choice="Enter your choice (1-3): "

if "%choice%"=="1" (
    set /p count="How many pump addresses? (default: 1): "
    if "%count%"=="" set count=1
    set /p batch="Batch size for uploads? (default: 10, 0=all at end): "
    if "%batch%"=="" set batch=10
    set /p local="Save local backup? (y/n, default: n): "
    if /i "%local%"=="y" (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- pump --count %count% --batch-size %batch% --save-local
    ) else (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- pump --count %count% --batch-size %batch%
    )
) else if "%choice%"=="2" (
    set /p count="How many bonk addresses? (default: 1): "
    if "%count%"=="" set count=1
    set /p batch="Batch size for uploads? (default: 10, 0=all at end): "
    if "%batch%"=="" set batch=10
    set /p local="Save local backup? (y/n, default: n): "
    if /i "%local%"=="y" (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- bonk --count %count% --batch-size %batch% --save-local
    ) else (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- bonk --count %count% --batch-size %batch%
    )
) else if "%choice%"=="3" (
    set /p count="How many addresses of each type? (default: 1): "
    if "%count%"=="" set count=1
    set /p batch="Batch size for uploads? (default: 10, 0=all at end): "
    if "%batch%"=="" set batch=10
    set /p local="Save local backup? (y/n, default: n): "
    if /i "%local%"=="y" (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- both --count %count% --batch-size %batch% --save-local
    ) else (
        echo 🚀 Running with maximum performance (all CPU cores)...
        cargo run --release -- both --count %count% --batch-size %batch%
    )
) else (
    echo Invalid choice!
    pause
    exit /b 1
)

echo.
echo Generation complete!
pause 