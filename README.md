# UltraArchiver 2.3

Native Windows 10/11 x64 archive manager written in Rust with a C++ Windows Explorer shell extension.

## Features
- Dark desktop interface with working Home, Archive, Extract and Settings pages.
- Embedded UltraArchiver brand mark and executable icon.
- Built-in ZIP, TAR and TAR.GZ create/extract engine.
- ZIP compression level 0–9, byte progress and input/output size report.
- Drag and drop files into the archive page.
- Standalone custom Rust/egui graphical installer (no Inno Setup).
- Optional desktop shortcut and Explorer context-menu registration.

## Build
Use a Windows x64 MSVC Rust toolchain and Visual Studio C++ Build Tools:
```powershell
cargo build --release --target x86_64-pc-windows-msvc --bin ultraarchiver
New-Item -ItemType Directory -Force dist
Copy-Item target\x86_64-pc-windows-msvc\release\ultraarchiver.exe dist\UltraArchiver.exe
cl /nologo /LD /EHsc /std:c++17 shell\UltraArchiverShell.cpp /link /OUT:dist\UltraArchiverShell.dll ole32.lib shell32.lib advapi32.lib user32.lib
cargo build --release --target x86_64-pc-windows-msvc --bin ultraarchiver-setup
```
The installer embeds the app and Explorer extension from `dist`. Generate the icon resources first with `assets/generate-brand.ps1`.

## Compression note
Already-compressed media (MP4, JPEG, many game assets) usually cannot be reduced substantially by ZIP/Deflate. Compression level changes CPU/time and may change output size only slightly for such data.

Build artifacts are published by the GitHub Actions workflow.
