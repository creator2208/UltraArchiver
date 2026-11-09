# UltraArchiver 2.4

Modern Windows 10/11 archive manager built with Rust and C++.

## Features
- Windows 11-inspired dark interface with animated UltraArchiver branding.
- Built-in ZIP, TAR and TAR.GZ support.
- **UltraPack (.uap)**: custom lossless container using Zstandard compression, BLAKE3 content hashes and duplicate-file elimination.
- Custom native GUI installer and optional Windows Explorer context menu integration.
- Drag-and-drop file selection and archive progress reporting.

## UltraPack notes
UltraPack tests each file with Zstandard and stores the original bytes when compression would make it larger. Identical files are stored once and referenced from the archive table. This can reduce archive size for collections with duplicate files. It cannot guarantee additional compression of already-compressed MP4, JPEG, MP3, ZIP or similar data; lossless compression cannot generally remove entropy that is already present.

## Build
On Windows with Rust stable, MSVC Build Tools and Windows SDK:
```powershell
cargo build --release --bin ultraarchiver
```
The Windows workflow builds the GUI application, Explorer extension and custom setup executable.
