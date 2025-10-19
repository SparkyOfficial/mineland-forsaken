# PowerShell script to compile and run the simple network demo
Write-Host "Compiling Simple Network Demo..."
rustc simple_network_demo.rs -o simple_network_demo.exe

if ($LASTEXITCODE -eq 0) {
    Write-Host "Compilation successful!"
    Write-Host "Running the demo..."
    Write-Host "=================="
    .\simple_network_demo.exe
} else {
    Write-Host "Compilation failed!"
}