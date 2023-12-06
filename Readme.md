

# Godot commands

Run project in editor:

```
godot godot/project.godot
cargo build ; godot godot/project.godot
```

Start scene in debug mode without editor (must be run from godot folder):

```
godot -d node_2d.tscn
```

# Deployment

ssh -i "./.ssh/cta-udp-real-time.pem" ubuntu@35.181.43.91