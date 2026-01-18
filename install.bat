@echo off
setlocal

:: REDLINE Installer for Windows

echo =======================================
echo  REDLINE Installer for Windows
echo =======================================
echo.

:: Set the default installation directory
set "INSTALL_DIR=%USERPROFILE%\.redline"

echo This script will install REDLINE to the following directory:
echo %INSTALL_DIR%
echo.
echo If this directory already exists, it will be deleted and replaced.
echo.

:PROMPT
set /p "CHOICE=Do you want to continue? (Y/N): "
if /i "%CHOICE%"=="y" goto :INSTALL
if /i "%CHOICE%"=="n" goto :CANCEL
goto :PROMPT

:INSTALL
echo.
echo --- Starting Installation ---

:: 1. Remove old installation if it exists
if exist "%INSTALL_DIR%" (
    echo Removing existing installation...
    rmdir /s /q "%INSTALL_DIR%"
)

:: 2. Copy the project files
echo Copying REDLINE source files...
:: Robocopy is robust and available on modern Windows systems
robocopy "%~dp0" "%INSTALL_DIR%" /E /NFL /NDL /NJH /NJS /nc /ns /np
if %errorlevel% geq 8 (
    echo Error: Failed to copy files. Aborting.
    goto :FAIL
)

:: 3. Add the installation directory to the User's PATH
echo Adding REDLINE to your PATH...
:: setx is the modern way to permanently modify environment variables
setx PATH "%%PATH%%;%INSTALL_DIR%"
if %errorlevel% neq 0 (
    echo.
    echo WARNING: Failed to automatically modify your PATH.
    echo You will need to add the following directory to your PATH manually:
    echo %INSTALL_DIR%
    echo.
)

:: 4. Initialize the compiler core
echo Initializing REDLINE compiler core...
call "%INSTALL_DIR%\redline.bat" init
if %errorlevel% neq 0 (
    echo Error: Failed to initialize compiler core.
    goto :FAIL
)

echo.
echo --- Installation Complete ---
echo.
echo REDLINE has been installed successfully.
echo Please restart your terminal for the 'redline' command to be available.
goto :END

:CANCEL
echo Installation cancelled.
goto :END

:FAIL
echo.
echo --- Installation Failed ---
echo Please check the error messages above.
goto :END

:END
endlocal
