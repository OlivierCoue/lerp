export $(grep -v '^#' .env | xargs)

cargo build -p lerp-client-game --target x86_64-pc-windows-gnu --release

cp ./target/x86_64-pc-windows-gnu/release/lerp-client-game.exe "${WINDOWS_EXPORT_PATH}/export"

