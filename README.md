# Vineflower Wrapper

A Rust-based command-line wrapper for the Vineflower Java decompiler that simplifies the process of decompiling Java class files and JAR archives.

## Overview

This tool provides a convenient wrapper around the [Vineflower](https://github.com/Vineflower/vineflower) decompiler, embedding the JAR file directly into the binary for easy distribution and execution. Vineflower is a modern Java decompiler forked from Fernflower, offering improved readability and better handling of modern Java features.

## Features

- **Self-contained**: Vineflower JAR is embedded in the binary - no external dependencies needed
- **Simple CLI**: Clean command-line interface with sensible defaults
- **Cross-platform**: Works on any system with Java installed
- **Automatic cleanup**: Temporary files are handled automatically

## Prerequisites

- Java Runtime Environment (JRE) 21 or higher must be installed and available in your system PATH
- The `java` command must be accessible from the command line

## Installation

### Building from Source

1. Clone this repository:
```bash
git clone https://github.com/t0bx/vineflower-wrapper
cd vineflower-wrapper
```

2. Ensure you have the Vineflower JAR file in the `assets/` directory:
```
assets/
└── vineflower.jar
```

3. Build the project:
```bash
cargo build --release
```

4. The binary will be available at `target/release/vineflower-wrapper.exe`

5. (Optional): Set up vineflower-wrapper into your system path:
```bash
cargo install --path .
```

## Usage

### Basic Syntax

```bash
vineflower-wrapper [OPTIONS] <input>
```

### Arguments

- `<input>` - Path to the Java class file, JAR file, or directory to decompile (required)

### Options

- `-o, --output <output>` - Output directory for decompiled files (default: "output")
- `-h, --help` - Print help information

### Examples

**Decompile a single class file:**
```bash
vineflower-wrapper MyClass.class
```

**Decompile a JAR file to a specific directory:**
```bash
vineflower-wrapper -o decompiled-source my-application.jar
```

## How It Works

1. The program extracts the embedded Vineflower JAR to a temporary location
2. Executes Java with the Vineflower JAR and your specified arguments
3. Vineflower processes your input and generates decompiled Java source files
4. Results are saved to the specified output directory

## Output

Decompiled Java source files will be created in the output directory, maintaining the original package structure. The tool will indicate success or failure of the decompilation process.

## Supported Input Formats

- Individual `.class` files
- JAR archives (`.jar`)

## Error Handling

The program will report errors in the following cases:
- Java is not installed or not found in PATH
- Input file/directory doesn't exist or isn't readable
- Vineflower decompilation fails
- Output directory cannot be created or written to

## Acknowledgments

- [Vineflower](https://github.com/Vineflower/vineflower) - The excellent Java decompiler this tool wraps
- [Clap](https://github.com/clap-rs/clap) - Command line argument parsing for Rust

## Troubleshooting

**"java: command not found"**
- Ensure Java is properly installed and added to your system PATH

**"Failed to decompile Java Class/File"**
- Check that your input file is a valid Java class file or JAR
- Verify you have read permissions for the input file
- Ensure the output directory is writable

**Build errors about missing vineflower.jar**
- Make sure `assets/vineflower.jar` exists in your project directory
- Download the latest Vineflower release if needed
