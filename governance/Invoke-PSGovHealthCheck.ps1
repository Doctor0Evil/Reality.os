. "$PSScriptRoot/PSGovernance.Types.ps1"
. "$PSScriptRoot/Invoke-PSGovPolicyEngine.ps1"

function Invoke-PSGovHealthCheck {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [uri]$EndpointUri,

        [Parameter(Mandatory)]
        [PSGovJurisdiction]$Jurisdiction,

        [Parameter(Mandatory)]
        [string]$PolicyOwner,

        [Parameter()]
        [string]$PolicyId = "RealityOs-AIL-GOV-h01",

        [Parameter()]
        [int]$RiskScore = 50
    )

    $hexTag = '5265416c6974794f732d41494c2d474f562d683031'

    $context = [PSGovPolicyContext]::new($PolicyId, $Jurisdiction, $PolicyOwner, $RiskScore)
    $context = Invoke-PSGovPolicyEngine -Context $context -OperationName 'HealthCheck'

    $logRoot = Join-Path $env:USERPROFILE 'PSGovernanceLogs'
    $null = New-Item -Path $logRoot -ItemType Directory -Force

    $logFile = Join-Path $logRoot 'RealityOs-AIL-GOV-h01.log'

    $checkResult = @{
        Timestamp     = [DateTime]::UtcNow.ToString('o')
        Endpoint      = $EndpointUri.AbsoluteUri
        Jurisdiction  = $context.Jurisdiction.ToString()
        PolicyId      = $context.PolicyId
        PolicyOwner   = $context.PolicyOwner
        RiskScore     = $context.RiskScore
        DecisionMode  = $context.DecisionMode.ToString()
        Notes         = $context.Notes
        HexTag        = $hexTag
        UserName      = $env:USERNAME
        Machine       = $env:COMPUTERNAME
    }

    try {
        if ($context.DecisionMode -eq [PSGovDecisionMode]::AutoExecute) {
            $response = Invoke-WebRequest -Uri $EndpointUri -UseBasicParsing -ErrorAction Stop
            $checkResult['StatusCode'] = $response.StatusCode
        }
        else {
            $checkResult['StatusCode'] = 'SkippedByPolicy'
        }
    }
    catch {
        $checkResult['StatusCode'] = 'Error'
        $checkResult['Error']      = $_.Exception.Message
    }

    ($checkResult | ConvertTo-Json -Compress) + "`n" | Out-File -FilePath $logFile -Encoding utf8 -Append

    return $checkResult
}
