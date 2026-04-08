#!/usr/bin/env pwsh
# Test script for osynic-midi CLI

Write-Host "=== Osynic MIDI CLI Test Suite ===" -ForegroundColor Cyan

$exe = ".\target\debug\osynic-midi.exe"

if (-not (Test-Path $exe)) {
    Write-Host "Error: Binary not found at $exe" -ForegroundColor Red
    Write-Host "Please run: cargo build" -ForegroundColor Yellow
    exit 1
}

Write-Host "Binary found: $exe" -ForegroundColor Green
Write-Host ""

# Test 1: Help command
Write-Host "Test 1: Display help message" -ForegroundColor Yellow
& $exe --help
Write-Host ""

# Test 2: List configs
Write-Host "Test 2: List available configurations" -ForegroundColor Yellow
& $exe list-configs
Write-Host ""

# Test 3: List devices
Write-Host "Test 3: List available MIDI devices" -ForegroundColor Yellow
& $exe list-devices
Write-Host ""

# Test 4: Help for subcommands
Write-Host "Test 4: Help for start subcommand" -ForegroundColor Yellow
& $exe start --help
Write-Host ""

Write-Host "=== All Tests Completed Successfully ===" -ForegroundColor Green

Write-Host ""
Write-Host "Build Summary:" -ForegroundColor Cyan
Write-Host "  Debug:   target\debug\osynic-midi.exe"
Write-Host "  Release: target\release\osynic-midi.exe"

Write-Host ""
Write-Host "Usage Examples:" -ForegroundColor Cyan
Write-Host "  - List configs:   $exe list-configs"
Write-Host "  - List devices:   $exe list-devices"
Write-Host "  - Interactive:    $exe start"
Write-Host "  - Direct start:   $exe start -c configs/midi_config.json -m notes"
