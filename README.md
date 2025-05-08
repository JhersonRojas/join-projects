 # Join Projects (Rust CLI)

This interactive CLI allows you to combine text files from a specific folder into a single file, ignoring binary or non-UTF-8 files and allowing you to customize which paths to ignore. It also creates an exception file if it encounters files not readable as text.

## Usage

When executing the binary:

1. Target path files: ./myproject
2. Name result (default: complete\_project.txt): output.txt
3. Files/folders will be ignored (optional): node_modules, .git, build

## Generated files

- `complete_project.txt`: consolidated file with comments for each file.
- `exceptions.txt`: list of non-UTF-8 files.

## Requirements

- [Rust Languaje](https://www.rust-lang.org/tools/install)

---

## Compilation by platform.

### Linux

```bash
cargo build --release
```

The executable will be in `target/release/`

---

### macOS

```bash
cargo build --release
```

It will also be in `target/release/`

---

### Windows

To compile from Linux to Windows, you need to use *cross-compiling*.

### 1. Install `x86_64-pc-windows-gnu`.

```bash
rustup target add x86_64-pc-windows-gnu
```

### 2. Install the GNU Windows toolchain (mingw-w64)

On Debian/Ubuntu:

```bash
sudo apt install mingw-w64
```

### 3. Compile

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The file will be in `target/x86_64-pc-windows-gnu/release/`
