# File: governance\Invoke-RealityOsGovernanceHealth.ps1
# Purpose: Lightweight, non-admin health and compliance probe for AI governance services

param(
    [string]$ServiceUrl = "https://governance.reality.os/health",
    [int]$TimeoutSeconds = 8
)

$timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssK"
$logRoot  = "$env:LOCALAPPDATA\RealityOs\GovernanceLogs"
$null = New-Item -ItemType Directory -Path $logRoot -Force

$logFile = Join-Path $logRoot "governance-health-$($timestamp.Substring(0,10)).log"

try {
    $response = Invoke-WebRequest -Uri $ServiceUrl -TimeoutSec $TimeoutSeconds -UseBasicParsing
    $status   = if ($response.StatusCode -eq 200) { "Healthy" } else { "Degraded" }
}
catch {
    $status   = "Unreachable"
}

$entry = [pscustomobject]@{
    Timestamp      = $timestamp
    GovernanceUrl  = $ServiceUrl
    Status         = $status
    Machine        = $env:COMPUTERNAME
    UserContext    = $env:USERNAME
}

$entry | ConvertTo-Json -Compress | Add-Content -Path $logFile
$entry
