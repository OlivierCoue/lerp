cargo build -p lerp-server-game --release

rm -rf ./lerp-server-game/dist/release
mkdir -p ./lerp-server-game/dist/release
cp ./lerp-server-game/Dockerfile ./lerp-server-game/dist/release
cp ./target/release/lerp-server-game ./lerp-server-game/dist/release

# docker rmi -f lerp-server-game:1
# docker build ./lerp-server-game/dist/release -t lerp-server-game:1
# docker run --rm -p 34255:34255/udp lerp-server-game:1