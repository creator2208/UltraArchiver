Add-Type -AssemblyName System.Drawing
New-Item -ItemType Directory -Force assets | Out-Null
function New-BrandBitmap($path,$w,$h,$small=$false) {
 $bmp=New-Object Drawing.Bitmap($w,$h);$g=[Drawing.Graphics]::FromImage($bmp);$g.SmoothingMode=[Drawing.Drawing2D.SmoothingMode]::AntiAlias
 $rect=New-Object Drawing.Rectangle(0,0,$w,$h);$brush=New-Object Drawing.Drawing2D.LinearGradientBrush($rect,[Drawing.Color]::FromArgb(10,22,43),[Drawing.Color]::FromArgb(15,75,145),45);$g.FillRectangle($brush,$rect)
 $scale=[Math]::Min($w,$h)*0.55;$x=($w-$scale)/2;$y=($h-$scale)/2
 $blue=New-Object Drawing.SolidBrush([Drawing.Color]::FromArgb(40,155,255))
 for($i=0;$i -lt 3;$i++){ $yy=$y+$i*$scale*0.27;$g.FillRectangle($blue,[single]$x,[single]$yy,[single]$scale,[single]($scale*0.20)) }
 $white=New-Object Drawing.SolidBrush([Drawing.Color]::White);$font=New-Object Drawing.Font("Segoe UI",([Math]::Max(7,$w*0.07)),[Drawing.FontStyle]::Bold)
 if(-not $small){$sf=New-Object Drawing.StringFormat;$sf.Alignment=[Drawing.StringAlignment]::Center;$g.DrawString("ULTRAARCHIVER",$font,$white,(New-Object Drawing.RectangleF(0,$h*0.78,$w,$h*0.12)),$sf)}
 $bmp.Save($path,[Drawing.Imaging.ImageFormat]::Bmp);$g.Dispose();$bmp.Dispose();$brush.Dispose()
}
New-BrandBitmap "assets\wizard.bmp" 164 314
New-BrandBitmap "assets\wizard-small.bmp" 55 55 $true
$bmp=New-Object Drawing.Bitmap(64,64);$g=[Drawing.Graphics]::FromImage($bmp);$g.SmoothingMode=[Drawing.Drawing2D.SmoothingMode]::AntiAlias;$b=New-Object Drawing.SolidBrush([Drawing.Color]::FromArgb(20,112,220));$g.FillRectangle($b,2,2,60,60);for($i=0;$i -lt 3;$i++){$g.FillRectangle([Drawing.Brushes]::White,12,($i*13+13),39,8)};$h=[IntPtr]::Zero;$h=$bmp.GetHicon();$ico=[Drawing.Icon]::FromHandle($h);$fs=[IO.File]::Create("assets\ultraarchiver.ico");$ico.Save($fs);$fs.Close();$ico.Dispose();$g.Dispose();$bmp.Dispose()
