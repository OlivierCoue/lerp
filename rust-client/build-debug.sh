export $(grep -v '^#' .env | xargs)

cargo build -p rust-client --target x86_64-pc-windows-gnu
cp ./target/x86_64-pc-windows-gnu/debug/rust_client.dll "${LERP_GODOT_LOCATION}/gdextension/debug"