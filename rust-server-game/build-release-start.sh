cargo build -p rust-server-game --release

rm -rf ./rust-server-game/dist/release
mkdir -p ./rust-server-game/dist/release
cp ./rust-server-game/Dockerfile ./rust-server-game/dist/release
cp ./target/release/rust-server-game ./rust-server-game/dist/release

docker rmi -f rust-server-game:1
docker build ./rust-server-game/dist/release -t rust-server-game:1
docker run --rm -p 34255:34255/udp rust-server-game:1