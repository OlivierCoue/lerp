export $(grep -v '^#' .env | xargs)

cargo build -p rust-client-game --target x86_64-pc-windows-gnu --release

cp ./target/x86_64-pc-windows-gnu/release/rust-client-game.exe "${LERP_GODOT_LOCATION}/export"

