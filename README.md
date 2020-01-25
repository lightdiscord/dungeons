# Dungeons

A work-in-progress project of a dungeon-minecraft-server written in rust.

## Development

### With nix

```
$ # $LOG_LEVEL will default to "trace" if not specified.
$ nix-shell --run "dev-watch $LOG_LEVEL"
```

### Without nix

```console
$ env RUST_LOG=$LOG_LEVEL cargo watch -x test -x run
```

TODO: Write more things
