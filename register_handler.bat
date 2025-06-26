@echo off
setlocal enabledelayedexpansion

echo aria2c-browse Protocol Handler Management
echo ========================================

if "%~1"=="" (
    echo.
    echo Usage: %0 path\to\aria2c-browse.exe
    echo.
    echo You must provide the path to aria2c-browse.exe
    echo.
    echo Examples:
    echo   %0 "C:\Users\username\aria2c-browse.exe"
    echo   %0 "C:\path\to\aria2c-browse.exe"
    echo.
    pause
    exit /b 1
)

set "EXE_PATH=%~1"

if not exist "!EXE_PATH!" (
    echo Error: Executable not found at "!EXE_PATH!"
    echo Please provide a valid path to aria2c-browse.exe
    pause
    exit /b 1
)

echo.
echo Validating executable...
"!EXE_PATH!" --help >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Executable validation failed. The file may be corrupted or incompatible.
    pause
    exit /b 1
)

echo.
echo Checking if aria2:// protocol handler is already registered...

reg query "HKEY_CLASSES_ROOT\aria2" >nul 2>&1
if %errorlevel% equ 0 (
    echo.
    echo Warning: aria2:// protocol handler is already registered.
    echo.
    set /p "choice=Do you want to delete the existing handler and continue? (y/N): "
    if /i "!choice!"=="y" (
        echo.
        echo Unregistering existing aria2:// protocol handler...
        reg delete "HKEY_CLASSES_ROOT\aria2" /f >nul 2>&1
        if %errorlevel% equ 0 (
            echo ✓ Existing handler unregistered.
        ) else (
            echo Error: Failed to unregister existing handler. Try running as Administrator.
            pause
            exit /b 1
        )
        echo.
    ) else (
        echo.
        echo Operation cancelled. Exiting.
        pause
        exit /b 0
    )
)

echo Registering aria2:// protocol handler...
echo Executable: !EXE_PATH!

echo.
echo Registry entries to be created:
echo   HKEY_CLASSES_ROOT\aria2
echo   HKEY_CLASSES_ROOT\aria2\shell\open\command
echo.

reg add "HKEY_CLASSES_ROOT\aria2" /ve /d "URL:aria2 Protocol" /f >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Failed to create aria2 registry key. Try running as Administrator.
    pause
    exit /b 1
)

reg add "HKEY_CLASSES_ROOT\aria2" /v "URL Protocol" /d "" /f >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Failed to set URL Protocol flag. Try running as Administrator.
    pause
    exit /b 1
)

reg add "HKEY_CLASSES_ROOT\aria2\shell" /ve /d "open" /f >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Failed to create shell registry key. Try running as Administrator.
    pause
    exit /b 1
)

reg add "HKEY_CLASSES_ROOT\aria2\shell\open" /ve /d "Open aria2 link" /f >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Failed to create open registry key. Try running as Administrator.
    pause
    exit /b 1
)

reg add "HKEY_CLASSES_ROOT\aria2\shell\open\command" /ve /d "\"!EXE_PATH!\" \"%%1\"" /f >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Failed to set command registry key. Try running as Administrator.
    pause
    exit /b 1
)

echo.
echo ✓ aria2:// protocol handler registered successfully!
echo.
echo Test with: aria2://browse/path=C%3A%5CUsers%5Ctest
echo Or run: "!EXE_PATH!" --help
echo.
echo To unregister later, run: unregister_handler.bat

pause 