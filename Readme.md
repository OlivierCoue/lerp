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

Build the game client:

```
./lerp-client-game/build-debug.sh
```

Build and start the game server:

```
./lerp-server-game/build-debug-start.sh
```