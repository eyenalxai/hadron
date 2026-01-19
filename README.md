# hadron

Launch Steam games with alternative executables through Proton.

## Usage

```bash
hadron <APP_ID> <EXE_PATH> [OPTIONS]
```

### Arguments

- `APP_ID` - Steam application ID (e.g., `1245620` for Elden Ring)
- `EXE_PATH` - Relative path to executable within game directory (e.g., `ersc_launcher.exe`)

### Options

- `-n, --dry-run` - Print command without executing
- `-u, --user-id <USER_ID>` - Steam user ID for launch options
- `-s, --steam-dir <PATH>` - Steam directory path (default: `~/.local/share/Steam`)

### Examples

Launch Elden Ring Seamless Co-op mod:
```bash
hadron 1245620 ersc_launcher.exe
```

Preview the command that would be executed:
```bash
hadron 1245620 ersc_launcher.exe --dry-run
```

Specify a custom Steam directory:
```bash
hadron 1245620 ersc_launcher.exe --steam-dir /mnt/games/Steam
```

Specify a Steam user ID (required when multiple Steam users exist):
```bash
hadron 1245620 ersc_launcher.exe --user-id 123456789
```

### Launch Options Preservation

hadron respects Steam launch options configured for the game, including complex setups like:
```bash
LD_PRELOAD= gamescope -f -H 1440 -h 1440 -r 75 --mangoapp -- env LD_PRELOAD="$LD_PRELOAD" gamemoderun %command%
```

The `%command%` placeholder is replaced with the Proton command for your alternative executable, ensuring all launch parameters are applied correctly.
