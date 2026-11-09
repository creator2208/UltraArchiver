Add-Type -AssemblyName System.Drawing
New-Item -ItemType Directory -Force assets | Out-Null
function Draw-Mark([Drawing.Graphics]$g,[single]$cx,[single]$cy,[single]$s) {
 $g.SmoothingMode=[Drawing.Drawing2D.SmoothingMode]::AntiAlias
 $navy=[Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(8,35,75))
 $ring=[Drawing.Pen]::new([Drawing.Color]::FromArgb(20,112,190),[single]($s*.075))
 $cyan=[Drawing.Pen]::new([Drawing.Color]::FromArgb(100,220,255),[single]($s*.025))
 $white=[Drawing.SolidBrush]::new([Drawing.Color]::White)
 $blue=[Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(45,165,245))
 $mid=[Drawing.SolidBrush]::new([Drawing.Color]::FromArgb(20,115,210))
 $g.FillEllipse($navy,$cx-$s*.47,$cy-$s*.47,$s*.94,$s*.94)
 $g.DrawEllipse($ring,$cx-$s*.43,$cy-$s*.43,$s*.86,$s*.86)
 $g.DrawEllipse($cyan,$cx-$s*.35,$cy-$s*.35,$s*.70,$s*.70)
 $g.FillRectangle($blue,$cx-$s*.19,$cy-$s*.27,$s*.38,$s*.13)
 $g.FillRectangle($blue,$cx-$s*.17,$cy-$s*.33,$s*.17,$s*.09)
 $font=[Drawing.Font]::new("Segoe UI",([single]($s*.48)),[Drawing.FontStyle]::Bold,[Drawing.GraphicsUnit]::Pixel)
 $fmt=[Drawing.StringFormat]::new();$fmt.Alignment=[Drawing.StringAlignment]::Center;$fmt.LineAlignment=[Drawing.StringAlignment]::Center
 $g.DrawString("U",$font,$white,[Drawing.RectangleF]::new($cx-$s*.30,$cy-$s*.14,$s*.60,$s*.55),$fmt)
 for($i=0;$i -lt 3;$i++){
  $yy=$cy+$s*(.13+$i*.075)
  $pts=[Drawing.PointF[]]@([Drawing.PointF]::new($cx-$s*.17,$yy),[Drawing.PointF]::new($cx+$s*.08,$yy-$s*.055),[Drawing.PointF]::new($cx+$s*.23,$yy+$s*.005),[Drawing.PointF]::new($cx-$s*.02,$yy+$s*.065))
  $g.FillPolygon($(if($i -eq 0){$blue}else{$mid}),$pts);$g.DrawPolygon([Drawing.Pen]::new([Drawing.Color]::FromArgb(185,235,255),[single]($s*.008)),$pts)
 }
 $arrow=[Drawing.PointF[]]@([Drawing.PointF]::new($cx+$s*.20,$cy-$s*.25),[Drawing.PointF]::new($cx+$s*.40,$cy-$s*.47),[Drawing.PointF]::new($cx+$s*.42,$cy-$s*.31),[Drawing.PointF]::new($cx+$s*.49,$cy-$s*.31),[Drawing.PointF]::new($cx+$s*.29,$cy-$s*.04),[Drawing.PointF]::new($cx+$s*.27,$cy-$s*.20))
 $g.FillPolygon($blue,$arrow);$g.DrawPolygon([Drawing.Pens]::White,$arrow)
 $fmt.Dispose();$font.Dispose();$white.Dispose();$blue.Dispose();$mid.Dispose();$navy.Dispose();$ring.Dispose();$cyan.Dispose()
}
function New-WizardBitmap([string]$path,[int]$w,[int]$h,[bool]$small=$false) {
 $bmp=[Drawing.Bitmap]::new($w,$h);$g=[Drawing.Graphics]::FromImage($bmp)
 $rect=[Drawing.Rectangle]::new(0,0,$w,$h)
 $brush=[Drawing.Drawing2D.LinearGradientBrush]::new($rect,[Drawing.Color]::FromArgb(5,20,43),[Drawing.Color]::FromArgb(14,83,155),45)
 $g.FillRectangle($brush,$rect)
 if($small){Draw-Mark $g ($w/2) ($h/2) ($w*.88)}else{Draw-Mark $g ($w/2) ($h*.35) ($w*.78)}
 $bmp.Save($path,[Drawing.Imaging.ImageFormat]::Bmp)
 $brush.Dispose();$g.Dispose();$bmp.Dispose()
}
New-WizardBitmap "assets\wizard.bmp" 164 314
New-WizardBitmap "assets\wizard-small.bmp" 55 55 $true
$bmp=[Drawing.Bitmap]::new(256,256);$g=[Drawing.Graphics]::FromImage($bmp);$g.Clear([Drawing.Color]::FromArgb(7,25,53));Draw-Mark $g 128 128 236
$h=$bmp.GetHicon();$ico=[Drawing.Icon]::FromHandle($h);$fs=[IO.File]::Create("assets\ultraarchiver.ico");$ico.Save($fs);$fs.Close();$ico.Dispose();$g.Dispose();$bmp.Dispose()
