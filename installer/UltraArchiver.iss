#define AppName "UltraArchiver"
#define AppVersion "2.2.0"
[Setup]
AppId={{A8BC2DF1-91EA-4F5C-9D4B-2A0A2C0A2000}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=UltraArchiver
AppPublisherURL=https://github.com/creator2208/UltraArchiver
DefaultDirName={autopf}\UltraArchiver
DefaultGroupName=UltraArchiver
OutputDir=..\installer-output
OutputBaseFilename=UltraArchiver-Setup-x64
ArchitecturesInstallIn64BitMode=x64
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
WizardSizePercent=115
WizardImageFile=..\assets\wizard.bmp
WizardSmallImageFile=..\assets\wizard-small.bmp
SetupIconFile=..\assets\ultraarchiver.ico
UninstallDisplayIcon={app}\UltraArchiver.exe
PrivilegesRequired=admin
DisableProgramGroupPage=yes
[Languages]
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"
[CustomMessages]
ukrainian.LaunchApp=Запустити UltraArchiver
ukrainian.DesktopIcon=Створити ярлик на робочому столі
ukrainian.Explorer=Додати команди до контекстного меню Провідника
ukrainian.WelcomeLabel1=Ласкаво просимо до UltraArchiver!
ukrainian.WelcomeLabel2=Потужне архівування у сучасному інтерфейсі Windows.
[Files]
Source: "..\dist\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
[Icons]
Name: "{autoprograms}\UltraArchiver"; Filename: "{app}\UltraArchiver.exe"
Name: "{autodesktop}\UltraArchiver"; Filename: "{app}\UltraArchiver.exe"; Tasks: desktopicon
[Tasks]
Name: "desktopicon"; Description: "{cm:DesktopIcon}"; Flags: unchecked
Name: "explorer"; Description: "{cm:Explorer}"; Flags: checkedonce
[Run]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s ""{app}\UltraArchiverShell.dll"""; Flags: runhidden; Tasks: explorer
Filename: "{app}\UltraArchiver.exe"; Description: "{cm:LaunchApp}"; Flags: postinstall nowait skipifsilent
[UninstallRun]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s /u ""{app}\UltraArchiverShell.dll"""; Flags: runhidden; RunOnceId: "UnregisterShell"
