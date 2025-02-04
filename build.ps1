[CmdletBinding()]
Param([switch]$Build, [switch]$Test)

$env:LIBCLANG_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib"

if ($Build) {
  Write-Host "Building..." -ForegroundColor Green
  cargo build *> ./target/result_build.txt
}
elseif ($Test) {
  Write-Host "Testing..." -ForegroundColor Green
  cargo test *> ./target/result_test.txt
}
else {
  Write-Host "Building..." -ForegroundColor Green
  cargo build *> ./target/result_build.txt
}

Write-Host "Done." -ForegroundColor Green
