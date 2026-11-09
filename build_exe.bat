@echo off
cd /d "%~dp0"
python -m pip install --upgrade pip pyinstaller
python -m PyInstaller --onefile --windowed --name UltraArchiver ultraarchiver.py
echo EXE готовий у папці dist
pause
