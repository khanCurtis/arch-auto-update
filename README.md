# Arch Auto-Update

A lightweight Rust utility that automatically updates your Arch Linux system on startup.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Arch Auto-Update is a simple, efficient tool designed to keep your Arch Linux system up-to-date with minimal user intervention. It runs automatically when your laptop starts up, ensuring your system receives critical security updates and package upgrades without manual action.

## Features

- **Automatic Updates**: Runs `pacman -Syu` automatically when your system boots
- **AUR Support**: Updates AUR packages using your AUR helper (yay by default)
- **Systemd Integration**: Easy setup with provided systemd service file
- **Minimal Resource Usage**: Written in Rust for efficiency and reliability
- **Zero Dependencies**: No external dependencies required

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/khanCurtis/arch-auto-update.git
   cd arch-auto-update
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary and service file:
   ```bash
   sudo cp target/release/arch-auto-update /usr/local/bin/
   sudo chmod +x /usr/local/bin/arch-auto-update
   sudo cp arch-auto-update.service /etc/systemd/system/
   ```

4. Enable the service:
   ```bash
   sudo systemctl enable arch-auto-update.service
   ```

5. Configure sudo permissions (see Security section)

## Configuration

### Customizing the AUR Helper

The default configuration uses `yay` as the AUR helper. If you use a different helper (like `paru`, `trizen`, etc.), modify the script in `src/main.rs`:

```rust
// Change this line:
let aur_result = Command::new("yay")
// To your preferred AUR helper:
let aur_result = Command::new("paru")
```

### Disabling AUR Updates

If you don't use AUR packages or prefer not to update them automatically, you can comment out or remove the AUR section from the script.

## Security Considerations

This tool requires sudo privileges to run updates. It's recommended to configure sudo specifically for this purpose:

```bash
sudo EDITOR=nano visudo -f /etc/sudoers.d/arch-auto-update
```

Add the following line (replace USERNAME with your username):
```
USERNAME ALL=(ALL) NOPASSWD: /usr/bin/pacman
```

If using an AUR helper, you may also need to add it to the sudoers file.

## Viewing Update Results

Since the tool doesn't use a dedicated log file, you can view update results in the systemd journal:

```bash
journalctl -u arch-auto-update.service
```

## FAQ

**Q: Is it safe to automatically update Arch Linux?**
A: While generally safe, automatic updates without supervision carry some risk. Consider implementing snapshot mechanisms before updates or checking the journal after updates for any issues.

**Q: Can I schedule updates instead of running at boot?**
A: Yes, you can modify the systemd service file to use a timer instead of running at boot.

**Q: What if an update breaks my system?**
A: Consider implementing a snapshot mechanism before updates, such as using Timeshift or Snapper to create system snapshots.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
