[CmdletBinding()]
Param([switch]$Build, [switch]$NTest, [switch]$Test, [switch]$Gen)

$env:LIBCLANG_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib"

if ($Build) {
  build
}
elseif ($Gen) {
  $env:RUST_BACKTRACE = 1
  Write-Host "Generate bindings..." -ForegroundColor Green
  cargo build --features "generate,vcpkg" --no-default-features *> ./target/gen_results.txt
}
elseif ($Test) {
  Write-Host "Testing..." -ForegroundColor Green
  # cargo test *> ./test_results.txt
  cargo test --features debug --no-default-features
}
elseif ($NTest) {
  Write-Host "Parallel Testing..." -ForegroundColor Green
  cargo nextest run --features debug --no-default-features
}
else {
  build
}

Write-Host "Done." -ForegroundColor Green

function build() {
  Write-Host "Building..." -ForegroundColor Green
  cargo build *> ./target/build_results.txt
}
