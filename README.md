<h1 align="center">
  raindom
</h1>

<p align="center">
  An RNG-LLM wrapper. Truly the intersection of cryptography and AI.
</p>

<div align="center">
  <a href="https://x.com/cryptograthor">
    <img src="https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink" />
    <!-- ![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink) -->
  </a>
  <a href="https://github.com/thor314/raindom/actions">
    <!-- ![](https://github.com/thor314/raindom/actions/workflows/ci.yml/badge.svg) -->
    <img src="https://github.com/thor314/raindom/actions/workflows/ci.yml/badge.svg" />
  </a>
  <!-- [![crates.io](https://img.shields.io/crates/v/raindom.svg)](https://crates.io/crates/raindom) -->
  <!-- [![Documentation](https://docs.rs/raindom/badge.svg)](https://docs.rs/raindom) -->
  </div>

# Raindom

Raindom is a CLI tool that generates random numbers using an LLM (Large Language Model) backend. It leverages Ollama's local AI models to generate random numbers.

## Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- Ollama (see installation instructions below)

## Installing Ollama

### macOS
```bash
brew install ollama
ollama serve
```

### Linux
We provide a setup script that installs Ollama and configures it as a systemd service:

```bash
# Run setup
# Warning: we need sudo to create ollama as a service, but you should probably not trust
# random parodies on the internet with sudo without at least looking at the script.
chmod +x setup-ollama.sh
sudo ./setup-ollama.sh
```

The script will:
- Install Ollama
- Configure it as a systemd service
- Pull required models
- Start the service automatically

## Installing Raindom
```bash
# Clone the repository
git clone https://github.com/yourusername/raindom.git
cd raindom

# Build and install. This puts the raindom cli in your path.
# The author will not upload this to crates.rs, as this is not a serious project.
cargo install --path .
```

## Usage
Raindom has three modes of operation:

1. Generate a random number between 0 and 10:
```bash
raindom
```

2. Generate a random number between 0 and max:
```bash
raindom 100  # Generates number between 0-100
```

3. Generate a random number between min and max:
```bash
raindom 50 100  # Generates number between 50-100
```

## Managing Ollama Service (Linux)

If you used our setup script, you can manage Ollama using systemctl:

```bash
# Check status
sudo systemctl status ollama

# Stop service
sudo systemctl stop ollama

# Start service
sudo systemctl start ollama

# Restart service
sudo systemctl restart ollama

# View logs
journalctl -u ollama -f
```

## Troubleshooting
1. If you get a connection error:
   - Check if Ollama is running:
     ```bash
     curl http://localhost:11434/api/generate -d '{"model": "mistral", "prompt": "hi"}'
     ```
   - On Linux, check service status:
     ```bash
     sudo systemctl status ollama
     ```

2. If you get model-related errors:
   - Ensure the Mistral model is installed:
     ```bash
     ollama pull mistral
     ```

3. For other issues, check Ollama logs:
   - Linux: `journalctl -u ollama -f`
   - macOS/Windows: Check the terminal where `ollama serve` is running

## How it Works
Raindom uses the Ollama API to query a local LLM (Mistral by default) to generate random numbers.

This is innovation was only made possible by the work of cryptographers and AI scientists.

This project mainly exists to lightly deride "the intersection of Cryptography and AI".

## License
Licensed under your option of either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments
- [Ollama](https://ollama.ai) for the local LLM runtime
- [ollama-rs](https://github.com/pepperoni21/ollama-rs) for the Rust bindings

