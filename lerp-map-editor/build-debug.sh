export $(grep -v '^#' .env | xargs)

cargo build -p lerp-map-editor --target x86_64-pc-windows-gnu --features "bevy/dynamic_linking"

cp ./target/x86_64-pc-windows-gnu/debug/lerp-map-editor.exe "${WINDOWS_EXPORT_PATH}"

for file in ./target/x86_64-pc-windows-gnu/debug/deps/bevy_dylib-*.dll; do
    file_name=$(basename "$file")

    if [ ! -f "${WINDOWS_EXPORT_PATH}/${file_name}" ]; then
        cp ./target/x86_64-pc-windows-gnu/debug/deps/${file_name} "${WINDOWS_EXPORT_PATH}"
    fi
done

for file in ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/std-*.dll; do
    file_name=$(basename "$file")

    if [ ! -f "${WINDOWS_EXPORT_PATH}/${file_name}" ]; then
        cp ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/${file_name} "${WINDOWS_EXPORT_PATH}"
    fi
done

cp -r ./lerp-map-editor/assets "${WINDOWS_EXPORT_PATH}/assets"