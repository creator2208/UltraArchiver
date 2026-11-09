@echo off
cd /d "%~dp0"
python ultraarchiver.py
if errorlevel 1 pause
