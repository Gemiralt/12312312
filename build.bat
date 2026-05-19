@echo off
title VertexOS Builder

echo ========================================
echo    VertexOS - Build System v2.0
echo    With Colors & Debugger
echo ========================================
echo.

if not exist "vertexvm\target\release\vertexvm.exe" (
    echo Building VM...
    cd vertexvm
    cargo build --release
    cd ..
    echo.
)

if not exist kernel\build mkdir kernel\build

echo Creating colorful kernel...
echo Colors: H(White) E(Red) L(Green) L(Blue) O(Yellow)

:: MOV EAX, 0xB8000
:: H - White (0x0F)
:: E - Red (0x04)
:: L - Green (0x02)
:: L - Blue (0x01)
:: O - Yellow (0x0E)
:: ! - Magenta (0x05)

powershell -Command "$bytes = [byte[]]@(0xB8,0x00,0x80,0x0B,0x00,0xC7,0x00,0x48,0x0F,0x00,0x00,0xC7,0x40,0x02,0x45,0x04,0x00,0x00,0xC7,0x40,0x04,0x4C,0x02,0x00,0x00,0xC7,0x40,0x06,0x4C,0x01,0x00,0x00,0xC7,0x40,0x08,0x4F,0x0E,0x00,0x00,0xC7,0x40,0x0A,0x21,0x05,0x00,0x00,0xF4); [System.IO.File]::WriteAllBytes('kernel\build\kernel.bin', $bytes)"

echo.
echo [Debug] Kernel size: 
dir kernel\build\kernel.bin | find "kernel.bin"

echo.
echo Starting VertexVM...
echo.
vertexvm\target\release\vertexvm.exe kernel\build\kernel.bin

echo.
echo Done!
pause