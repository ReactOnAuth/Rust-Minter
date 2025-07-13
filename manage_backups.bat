@echo off
echo ===============================================
echo Solana Mint Address Generator - Backup Manager
echo ===============================================
echo.
echo Available backup management commands:
echo 1. List all backup files
echo 2. View backup file contents
echo 3. Import backup to Supabase
echo 4. Clean old backups
echo 5. Convert backup to different format
echo.
set /p choice="Enter your choice (1-5): "

if "%choice%"=="1" (
    echo.
    echo 📁 Backup files in current directory:
    dir *_addresses_*.txt /b 2>nul
    if errorlevel 1 (
        echo No backup files found.
    )
) else if "%choice%"=="2" (
    echo.
    echo Available backup files:
    dir *_addresses_*.txt /b 2>nul
    echo.
    set /p filename="Enter filename to view: "
    if exist "%filename%" (
        echo.
        echo Contents of %filename%:
        echo =====================
        type "%filename%"
    ) else (
        echo File not found!
    )
) else if "%choice%"=="3" (
    echo.
    echo 🔄 Import backup to Supabase
    echo This feature would require additional implementation
    echo to parse CSV files and upload to database.
    echo.
    echo For now, you can:
    echo 1. Use the main generator with --batch-size 0 to avoid duplicates
    echo 2. Manually import using Supabase dashboard
) else if "%choice%"=="4" (
    echo.
    set /p days="Delete backup files older than how many days? (default: 7): "
    if "%days%"=="" set days=7
    echo.
    echo 🗑️  This would delete backup files older than %days% days
    echo Implementation: forfiles /p . /m *_addresses_*.txt /d -%days% /c "cmd /c del @path"
    echo.
    set /p confirm="Are you sure? (y/n): "
    if /i "%confirm%"=="y" (
        forfiles /p . /m *_addresses_*.txt /d -%days% /c "cmd /c del @path" 2>nul
        echo Done!
    ) else (
        echo Cancelled.
    )
) else if "%choice%"=="5" (
    echo.
    echo 📋 Convert backup formats
    echo Available backup files:
    dir *_addresses_*.txt /b 2>nul
    echo.
    set /p filename="Enter filename to convert: "
    if exist "%filename%" (
        echo.
        echo Converting %filename% to JSON format...
        set "output=%filename:.txt=.json%"
        echo [ > "%output%"
        for /f "tokens=1,2,3 delims=," %%a in (%filename%) do (
            echo   {"pub_key": "%%a", "private_key": "%%b", "suffix_type": "%%c"}, >> "%output%"
        )
        echo ] >> "%output%"
        echo Converted to %output%
    ) else (
        echo File not found!
    )
) else (
    echo Invalid choice!
)

echo.
pause 