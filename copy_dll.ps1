$projectRoot = (Get-Location).Path
$sourceDll = "$projectRoot\dll\libintl-9.dll"
$destinationDebug = "$projectRoot\packages\api\target\debug"
$destinationRelease = "$projectRoot\packages\api\target\release"

# Create destination directories if they don't exist (preferred approach)
If (!(Test-Path $destinationDebug)) {
    New-Item -ItemType Directory -Force -Path $destinationDebug
}
If (!(Test-Path $destinationRelease)) {
    New-Item -ItemType Directory -Force -Path $destinationRelease
}

# Copy the DLL to the created directories
Copy-Item $sourceDll $destinationDebug -Force
Copy-Item $sourceDll $destinationRelease -Force
