<#
What I learned
Much like C#, PowerShell's for statement doesn't require the condition section.
Accessing System.Math in PowerShell is ugly.
#>
[double]$Phi = [double]((1 + [System.Math]::Sqrt(5)) / 2)

$sum = 0

for ($i = 3;; $i+=3) {
    [int]$value = [int](([System.Math]::Pow($Phi, $i) - (-[System.Math]::Pow($Phi, -$($i)))) / [System.Math]::Sqrt(5))

    if ($value -gt 4000000) {
        break
    }

    $sum += $value
}

$sum
