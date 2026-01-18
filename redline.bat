@echo off
:: REDLINE Batch Wrapper for Windows

:: Find the directory where this script is located
set SCRIPT_DIR=%~dp0

:: Call the Python script, forwarding all arguments
python "%SCRIPT_DIR%redline.py" %*
