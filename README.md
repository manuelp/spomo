# spomo
Simple CLI Pomodoro timer.

## Install
Use the provided `install.sh` script to build the release binary and install it in `$HOME/bin`.

To do it manually:

1. Run: `cargo build --release`
2. Copy the `target/release/spomo` binary wherever you like

## Usage
Run the `spomo` command providing the duration(s) as an argument with the following syntax:

```
[num][smh]
```

For example:

- `10s`: 10 seconds
- `25m`: 25 minutes
- `1h`: 1 hour

You can provide multiple durations, they will be added together and used as the duration. For example:

- `5m 12s`: 5 minutes + 12 seconds

When the duration ends, a sound will be played and the program will quit, printing the end timestamp.

When the timer is running, you can press `q` to quit the program.