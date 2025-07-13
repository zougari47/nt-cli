# NT CLI

A minimal, lightweight CLI tool to write notes without leaving your terminal.

## Features

- Create notes with timestamp and current directory context
- Show and delete notes
- Note ideas without leaving the terminal. Stay focused on your task

## Installation (Linux)

### With Rust Installed

If you have Rust and Cargo installed, you can build and install `nt` directly from the source:

⚠️ Ensure `~/.local/bin` is in your `$PATH`.

```bash
git clone https://github.com/zougari47/nt-cli.git
cd nt
cargo build --release
cp target/release/nt ~/.local/bin/
```

### Without Rust Installed

If you don't have Rust, download the latest precompiled binary from the [Releases](https://github.com/zougari47/nt-cli/releases/) page.

⚠️ Ensure `~/.local/bin` is in your `$PATH`.

```bash
unzip nt-cli-v0.1.0-linux.zip
chmod +x nt
mv nt ~/.local/bin/
```

## Usage

- Add new note

```bash
nt "Save this idea for later"
```

- Show notes

```bash
nt -s
## OR
nt --show
```

- Delete all notes

```bash
nt -d
## OR
nt --delete
```
