cargo build -p rust-server-game

cp ./target/debug/rust-server-game ./rust-server-game/image

docker rmi -f rust-server-game:1
docker build ./rust-server-game/image -t rust-server-game:1

kubectl delete pods -n lerp -l app=game-server --force --grace-period 0
kubectl delete services -n lerp -l app=game-server