cargo build -p rust-server-game --features bevy/dynamic_linking

rm -rf ./rust-server-game/dist/debug
mkdir -p ./rust-server-game/dist/debug
cp ./rust-server-game/Dockerfile ./rust-server-game/dist/debug
cp ./target/debug/rust-server-game ./rust-server-game/dist/debug
cp $(find target -name "libbevy_dylib-*.so") ./rust-server-game/dist/debug
cp ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-*.so ./rust-server-game/dist/debug

docker rmi -f rust-server-game:1
docker build ./rust-server-game/dist/debug -t rust-server-game:1
docker run --rm -p 34000-35000:34000-35000/udp -p 4000:4000/tcp rust-server-game:1