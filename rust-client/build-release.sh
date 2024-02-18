export $(grep -v '^#' .env | xargs)

cargo build -p rust-client --release --target x86_64-pc-windows-gnu
cp ./target/x86_64-pc-windows-gnu/release/rust_client.dll "${LERP_GODOT_LOCATION}/gdextension/release"