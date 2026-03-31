# Installation

Seisly is available as pre-built binaries for Windows, Linux, and macOS. You can also build it from source if you prefer.

## Pre-built Binaries

Visit the [Seisly Releases](https://github.com/seisly/seisly/releases) page on GitHub to download the latest installer or archive for your platform.

- **Windows**: Download and run the `.msi` or `.exe` installer.
- **Linux**: Download the `.deb` or `.rpm` package, or the standalone binary.
- **macOS**: Download and run the `.pkg` or `.dmg` installer.

## Build from Source

To build Seisly from source, you need the [Rust toolchain](https://rustup.rs/) installed.

### System Dependencies

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  libssl-dev \
  pkg-config \
  libgtk-3-dev \
  libfontconfig1-dev \
  protobuf-compiler
```

#### Windows

For Windows, you'll need the following installed (e.g., via [Chocolatey](https://chocolatey.org/)):

```powershell
choco install protoc sqlite
```

#### macOS

Install dependencies via [Homebrew](https://brew.sh/):

```bash
brew install openssl pkg-config
```

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/seisly/seisly.git
   cd seisly
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in `target/release/seisly-app`.

## Python Package

Seisly's Python bindings can be installed via `pip`:

```bash
pip install seisly
```

Ensure you have the necessary Rust and system dependencies installed if building from source.
