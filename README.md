# criterion-history

Plot historical criterion benchmark tests.

Requires using the following PowerShell script (or comparable) to run benchmark tests from the project root:
```powershell
$timestamp = Get-Date -Format "yyyyMMddHHmm"
cargo bench -- --save-baseline BM_$timestamp
```

![example plot](https://raw.githubusercontent.com/PTaylor-us/criterion-history/master/plot.svg)