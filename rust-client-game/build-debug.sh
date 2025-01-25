export $(grep -v '^#' .env | xargs)

cargo build -p rust-client-game --target x86_64-pc-windows-gnu --features "bevy/dynamic_linking"

cp ./target/x86_64-pc-windows-gnu/debug/rust-client-game.exe "${LERP_GODOT_LOCATION}/export"

for file in ./target/x86_64-pc-windows-gnu/debug/deps/bevy_dylib-*.dll; do
    file_name=$(basename "$file")

    if [ ! -f "${LERP_GODOT_LOCATION}/export/${file_name}" ]; then
        cp ./target/x86_64-pc-windows-gnu/debug/deps/${file_name} "${LERP_GODOT_LOCATION}/export"
    fi
done

for file in ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/std-*.dll; do
    file_name=$(basename "$file")

    if [ ! -f "${LERP_GODOT_LOCATION}/export/${file_name}" ]; then
        cp ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/${file_name} "${LERP_GODOT_LOCATION}/export"
    fi
done