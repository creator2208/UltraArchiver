#define AppName "UltraArchiver"
#define AppVersion "2.0.0"
[Setup]
AppId={{A8BC2DF1-91EA-4F5C-9D4B-2A0A2C0A2000}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=UltraArchiver
DefaultDirName={autopf}\UltraArchiver
DefaultGroupName=UltraArchiver
OutputDir=..\installer-output
OutputBaseFilename=UltraArchiver-Setup-x64
ArchitecturesInstallIn64BitMode=x64
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\UltraArchiver.exe
PrivilegesRequired=admin
[Files]
Source: "..\dist\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\UltraArchiverShell.dll"; DestDir: "{app}"; Flags: ignoreversion
[Icons]
Name: "{autoprograms}\UltraArchiver"; Filename: "{app}\UltraArchiver.exe"
Name: "{autodesktop}\UltraArchiver"; Filename: "{app}\UltraArchiver.exe"; Tasks: desktopicon
[Tasks]
Name: "desktopicon"; Description: "Створити ярлик на робочому столі"; Flags: unchecked
Name: "explorer"; Description: "Додати команди до контекстного меню Провідника"; Flags: checkedonce
[Run]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s ""{app}\UltraArchiverShell.dll"""; Flags: runhidden; Tasks: explorer
[UninstallRun]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s /u ""{app}\UltraArchiverShell.dll"""; Flags: runhidden; RunOnceId: "UnregisterShell"
