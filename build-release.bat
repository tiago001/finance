@REM ## configure these for your environment
@REM # cargo package name
set PKG="finance"
@REM # remote target
set TARGET="x86_64-unknown-linux-gnu"
@REM # list of assets to bundle
set ASSETS=("static" "templates")
@REM # cargo build directory
set BUILD_DIR="target/%TARGET%/release/"

@REM ## ensure target toolchain is present
rustup target add %TARGET%

timeout 1
@REM ## cross-compile
@REM cargo zigbuild --target %TARGET% --release
cross build --target %TARGET% --release

timeout 4

@REM ## bundle
tar -cvzf "%PKG%.tar.gz" "static" "templates" -C "%BUILD_DIR%" "%PKG%"
