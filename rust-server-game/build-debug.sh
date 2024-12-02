cargo build -p rust-server-game

cp ./target/debug/rust-server-game ./rust-server-game/docker

docker rmi -f rust-server-game:1
docker build ./rust-server-game/docker -t rust-server-game:1
docker run --rm -p 34255:34255/udp rust-server-game:1