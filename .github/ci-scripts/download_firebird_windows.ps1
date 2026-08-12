$repoApiUrl = "https://api.github.com/repos/FirebirdSQL/firebird/releases/latest"
$outputFolder = "C:\Program Files\Firebird"
$assetNamePattern = "*-windows-x64.zip"

New-Item -Path $outputFolder -ItemType "Directory"

# Try with certificate check first, fallback to skipping if it fails
try {
    $release = Invoke-RestMethod -Uri $repoApiUrl -SslProtocol Tls12
} catch {
    Write-Host "First attempt failed: $_"
    Write-Host "Retrying with -SkipCertificateCheck..."
    try {
        $release = Invoke-RestMethod -Uri $repoApiUrl -SslProtocol Tls12 -SkipCertificateCheck
    } catch {
        Write-Error "Error get releases: $_"
        exit
    }
}

$asset = $release.assets | Where-Object { $_.name -like $assetNamePattern } | Select-Object -First 1

if (-not $asset) {
    Write-Host "File by asset '$assetNamePattern' not found."
    exit
}

$downloadUrl = $asset.browser_download_url
$outputPath = Join-Path $outputFolder $asset.name

Write-Host "Download $($asset.name)..."
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $outputPath -SslProtocol Tls12
    Write-Host "File successfully written: $outputPath"
} catch {
    Write-Host "Download with cert check failed: $_"
    Write-Host "Retrying with -SkipCertificateCheck..."
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $outputPath -SslProtocol Tls12 -SkipCertificateCheck
        Write-Host "File successfully written: $outputPath"
    } catch {
        Write-Error "Error download file: $_"
        exit
    }
}

Expand-Archive -Path $outputPath -DestinationPath $outputFolder -Force

Remove-Item $outputPath

Set-Location -Path $outputFolder

$currentPath = Get-Location

$runServiceFilename = "./install_service.bat"
& "$runServiceFilename"

Set-Location $currentPath
