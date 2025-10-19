@echo off
echo Building Network Demo...
rustc --crate-type bin -o network_demo.exe network_demo.rs --extern serde=serde --extern bincode=bincode
if %errorlevel% == 0 (
    echo Build successful!
    echo Run network_demo.exe to test the networking functionality
) else (
    echo Build failed!
)