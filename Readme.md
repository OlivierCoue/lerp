# Lerp

## Run project localy


### Requirement (windows)

Docker Desktop: https://docs.docker.com/desktop/install/windows-install/.

WSL: https://learn.microsoft.com/en-us/windows/wsl/install

VsCode: https://code.visualstudio.com/

### Build and start

Open project in VsCode DevContainer.

Copy .env.example to .env (update the copied version if needed):

```
cp .env.example .env
```

Build:

```
cargo build
```

Start rust-server-auth:

```
cargo lambda watch -a 0.0.0.0 -p 3000 --ignore-changes
// http://127.0.0.1:3000/lambda-url/rust-server-auth
```

Start rust-server-game:

```
cargo run -p rust-server-game
./rust-server-game/build-debug.sh
```

Build rust-client for windows (first, set the LERP_GODOT_LOCATION variable in your .env)

```
./rust-client/build-debug.sh
./rust-client/build-release.sh
```

## SQLX

Generate query metadata file (.sqlx) (require https://crates.io/crates/sqlx-cli to be installed)

See doc to add it as a pre-commit hook: https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-do-i-compile-with-the-macros-without-needing-a-database-eg-in-ci

```
cargo sqlx prepare --workspace
```

## Workaround: windows cross compile

Try once `./rust-client/build-debug.sh`

Then change in the `win32.c`, add include `ws2tcpip.h`

## K8S Setup

```
kubectl create namespace lerp
kubectl create namespace ingress-nginx
kubectl create configmap nginx-custom --from-file=nginx-tmpl=nginx.tmpl --namespace ingress-nginx

helm upgrade -f ./ingress-nginx-values.yaml --install ingress-nginx ingress-nginx \
  --repo https://kubernetes.github.io/ingress-nginx \
  --namespace ingress-nginx
```

## K8S Remove all

```
helm uninstall ingress-nginx --namespace ingress-nginx
kubectl delete configmaps nginx-custom --namespace ingress-nginx
```