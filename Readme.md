# Lerp

## Run project localy


### Requirement (windows)

Rust: https://www.rust-lang.org/tools/install.

Docker Desktop: https://docs.docker.com/desktop/install/windows-install/.

Godot (4.2.1): https://godotengine.org/download/windows/. You need to add the installation location to your path in order to use the `godot` command in your terminal.

Scoop (windows package manager: https://scoop.sh/):

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
```

Cargo lambda (https://github.com/awslabs/aws-lambda-rust-runtime):

```
scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
scoop install cargo-lambda/cargo-lambda
```

### Build and start

Copy .env.example to .env (update the copied version if needed):

```
cp .env.example .env
```

Start docker:

```
docker-compose up -d
```

Build:

```
cargo build
```

Start rust-server-auth:

```
./rust-server-auth cargo lambda watch -a 127.0.0.1 -p 3000
```

Start rust-server-game:

```
cargo run -p rust-server-game
```

Start client (in godot editor):

```
godot godot/project.godot
```

## Architecture (wip)

```
godot
rust-client
rust-common
rust-server-common
rust-server-auth
rust-server-lobby
rust-server-game
```








