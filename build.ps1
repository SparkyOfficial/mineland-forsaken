# Build script for Mineland Forsaken
# author: Андрій Будильников

Write-Host "Building Mineland Forsaken..." -ForegroundColor Green
cargo build

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
    Write-Host "To run the game, use: cargo run" -ForegroundColor Yellow
} else {
    Write-Host "Build failed!" -ForegroundColor Red
}