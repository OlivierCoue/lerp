export $(grep -v '^#' .env | xargs)

cargo build -p lerp-client-game --target x86_64-pc-windows-gnu --features "bevy/dynamic_linking"

cp ./target/x86_64-pc-windows-gnu/debug/lerp-client-game.exe "${WINDOWS_EXPORT_PATH}"

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

if [ ! -f "${WINDOWS_EXPORT_PATH}/libgcc_s_seh-1.dll" ]; then
    cp /usr/lib/gcc/x86_64-w64-mingw32/10-posix/libgcc_s_seh-1.dll "${WINDOWS_EXPORT_PATH}"
fi

if [ ! -f "${WINDOWS_EXPORT_PATH}/libwinpthread-1.dll" ]; then
    cp /usr/x86_64-w64-mingw32/lib/libwinpthread-1.dll "${WINDOWS_EXPORT_PATH}"
fi


cp -r ./lerp-client-game/assets "${WINDOWS_EXPORT_PATH}/assets"