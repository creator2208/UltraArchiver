Add-Type -AssemblyName System.Drawing
New-Item -ItemType Directory -Force assets | Out-Null
function New-BrandBitmap([string]$path,[int]$w,[int]$h) {
 $bmp=[Drawing.Bitmap]::new($w,$h);$g=[Drawing.Graphics]::FromImage($bmp)
 $rect=[Drawing.Rectangle]::new(0,0,$w,$h)
 $brush=[Drawing.Drawing2D.LinearGradientBrush]::new($rect,[Drawing.Color]::FromArgb(8,20,40),[Drawing.Color]::FromArgb(18,83,160),45)
 $g.FillRectangle($brush,$rect)
 $blue=[Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(30,145,255))
 $scale=[Math]::Min($w,$h)*0.5;$x=($w-$scale)/2;$y=($h-$scale)/2
 for($i=0;$i -lt 3;$i++){ $g.FillRectangle($blue,[single]$x,[single]($y+$i*$scale*0.28),[single]$scale,[single]($scale*0.20)) }
 $white=[Drawing.SolidBrush]::new([Drawing.Color]::White)
 $g.FillRectangle($white,[single]($x+$scale*0.76),[single]($y+$scale*0.1),[single]($scale*0.06),[single]($scale*0.75))
 $bmp.Save($path,[Drawing.Imaging.ImageFormat]::Bmp)
 $white.Dispose();$blue.Dispose();$brush.Dispose();$g.Dispose();$bmp.Dispose()
}
New-BrandBitmap "assets\wizard.bmp" 164 314
New-BrandBitmap "assets\wizard-small.bmp" 55 55
$bmp=[Drawing.Bitmap]::new(64,64);$g=[Drawing.Graphics]::FromImage($bmp)
$g.Clear([Drawing.Color]::FromArgb(12,35,68));$blue=[Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(30,145,255))
for($i=0;$i -lt 3;$i++){$g.FillRectangle($blue,10,10+$i*15,42,10)}
$h=$bmp.GetHicon();$ico=[Drawing.Icon]::FromHandle($h);$fs=[IO.File]::Create("assets\ultraarchiver.ico");$ico.Save($fs);$fs.Close()
$ico.Dispose();$blue.Dispose();$g.Dispose();$bmp.Dispose()
