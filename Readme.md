# Lerp

## Run project localy

Copy .env.example:

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

Start server:

```
cargo run -p rust-server
```

Start client (in godot editor):

```
godot godot/project.godot
```

## Architecture

```
godot
rust-client
rust-common
rust-server-common
rust-server-auth
rust-server-lobby
rust-server-game
```

## Deployment

ssh -i "./.ssh/cta-udp-real-time.pem" ubuntu@35.181.43.91








