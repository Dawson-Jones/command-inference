# cmdi

AI-powered shell command fixer. Type an approximate command, get the corrected version.

## Install

```bash
cargo install cmd-infer
```

## Usage

```bash
# Set your DeepSeek API key
export CMDI_KEY="your-key-here"

# Simple command (args joined automatically)
cmdi grep too high

# Command with shell metacharacters (wrap in quotes)
cmdi 'zcat file.zip | grep "too high"'
```

`cmdi` prints the corrected command to stdout. Copy and run it yourself.

## Configuration

| Environment Variable | Description |
|---------------------|-------------|
| `CMDI_KEY` | DeepSeek API key (required) |

## How it works

Sends your approximate command to DeepSeek (deepseek-v4-flash) and returns the corrected version. No execution, no side effects.
