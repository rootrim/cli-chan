# 🌺 cli-chan

Anime girl but with a command line interface.  
Supports Gemini API *for now*.  

Current Progress:  
- [x] Basic api usage.
- [ ] TOML config file.
    - [ ] Multiple personality support.
- [ ] History.
- [ ] Extended functionality.
- [ ] Extended api usage.

## How to install

### Source:

```bash
git clone https://github.com/LiniHub/cli-chan
cd cli-chan
cargo build --release
./target/release/cli-chan help
```

### Nix

```nix
inputs.cli-chan.url = "github:LiniHub/cli-chan";
```

```nix
environment.systemPackages = let
  system = pkgs.stdenv.hostPlatform.system;
in [
  inputs.cli-chan.packages.${system}.default
];
```

## Usage

```txt
A cli AI anime girl app for those who live in terminal.

Usage: cli-chan [OPTIONS] <COMMAND>

Commands:
  send  Send message to cli-chan
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --api-key <API_KEY>  API key
  -h, --help               Print help
  -V, --version            Print version
```

```bash
# use --api-key to set the api key inline or add it as GEMINI_API_KEY environment variable
cli-chan send "what does くりかえす mean, do not use any markdown" 
cli-chan help
```
