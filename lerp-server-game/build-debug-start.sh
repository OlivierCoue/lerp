cargo build -p lerp-server-game --features bevy/dynamic_linking

rm -rf ./lerp-server-game/dist/debug
mkdir -p ./lerp-server-game/dist/debug
cp ./lerp-server-game/Dockerfile ./lerp-server-game/dist/debug
cp ./target/debug/lerp-server-game ./lerp-server-game/dist/debug
cp $(find target -name "libbevy_dylib-*.so") ./lerp-server-game/dist/debug
cp ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-*.so ./lerp-server-game/dist/debug

docker rmi -f lerp-server-game:1
docker build ./lerp-server-game/dist/debug -t lerp-server-game:1
docker run --rm -p 34000-34005:34000-34005/udp -p 4000:4000/tcp lerp-server-game:1