# NanoFetch

A blazing fast, minimal terminal fetch and daily quote utility written in Rust.

```text
  user  gourav

      "You do not rise to the level of your goals.
       You fall to the level of your systems."
```

## Features

- **Daily Quote**: Displays one quote per day, remaining consistent across all terminal sessions throughout the day.
- **365 Curated Quotes**: Contains exactly 365 unique, thoughtful quotes on craft, philosophy, discipline, systems, and mindset—one for every day of the year.
- **Anti-Repetition**: Automatically avoids repeating yesterday's quote when the day rolls over.
- **Ultra-Fast & Minimal**: Pure Rust with zero external dependencies, sub-millisecond execution, and no unnecessary UI bloat or borders.
- **Easily Extensible**: Add custom quotes directly in code or in `~/.config/nanofetch/quotes.txt`.

## Installation & Build

### Using Cargo

```bash
cargo build --release
cp target/release/nanofetch ~/.local/bin/
```

### Using Make

```bash
make
make setup   # copies to ~/.local/bin
```

## Usage

Run directly or add to your `~/.zshrc` / `~/.bashrc`:

```bash
nanofetch
```

### Command Options

```text
nanofetch [OPTIONS]

OPTIONS:
    -r, --refresh    Refresh quote for today
    -h, --help       Print help information
    -V, --version    Print version information
```

## Custom Quotes

Create `~/.config/nanofetch/quotes.txt` and add one quote per line. Multi-line quotes can use `\n`:

```text
The only way to do great work is to love what you do.
Stay hungry.\nStay foolish.
```
