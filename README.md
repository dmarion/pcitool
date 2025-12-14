# pcitool

`pcitool` is a modern, interactive PCI diagnostics helper for Linux. It provides a terminal-based user interface (TUI) to explore the PCI topology, decode configuration space headers, and drill down into standard and extended capabilities with human-readable descriptions.

## Installation

### From Source

```bash
git clone https://github.com/dmarion/pcitool.git
cd pcitool
cargo install --path .
```

*Note: Accessing PCI configuration space typically requires root privileges (e.g., `sudo pcitool`).*

## Usage

### Interactive Mode
Simply run without arguments to browse all devices:
```bash
sudo pcitool
```

### Direct Inspection
Specify one or more devices to jump straight to them:
```bash
sudo pcitool --address 0000:01:00.0
```

### Dump Mode
Output the device tree directly to the terminal:
```bash
sudo pcitool --address 0000:01:00.0 --dump
```

## License

This project is licensed under the **Apache License, Version 2.0**. See the [LICENSE](LICENSE) file for details.
