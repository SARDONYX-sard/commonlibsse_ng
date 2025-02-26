[CmdletBinding()]
Param(
  [switch]$Build,
  [switch]$NTest,
  [switch]$Test,
  [switch]$Gen,
  [switch]$Example
)

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
elseif ($Example) {
  Write-Host "Running example..." -ForegroundColor Green
  cargo build --example module_state --features "tracing,no_sys,win_api"
  # $dest_dir = "./build/mods/module_state_example/SKSE/Plugins/";
  $dest_dir = "D:\GAME/ModOrganizer Skyrim SE/mods/module_state_example/SKSE/Plugins/"
  New-Item -ErrorAction SilentlyContinue -InformationAction SilentlyContinue -ItemType Directory $dest_dir
  Copy-Item -Path "./target/debug/examples/module_state.dll" -Destination $dest_dir -Force
  Copy-Item -Path "./target/debug/examples/module_state.pdb" -Destination $dest_dir -Force
}
else {
  build
}

Write-Host "Done." -ForegroundColor Green

function build() {
  Write-Host "Building..." -ForegroundColor Green
  cargo build *> ./target/build_results.txt
}
