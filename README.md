# Arch Auto-Update

A lightweight Rust utility that automatically updates your Arch Linux system on startup with a visible terminal window.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Arch Auto-Update is a simple, efficient tool designed to keep your Arch Linux system up-to-date with minimal user intervention. It runs automatically when your laptop starts up, opening a terminal window that shows the update process in real-time. After updates complete, the terminal automatically closes.

## Features

- **Automatic Updates**: Runs `pacman -Syu` automatically when your system boots
- **AUR Support**: Updates AUR packages using your AUR helper (yay by default)
- **Visual Feedback**: Opens a terminal window to show update progress
- **Auto-closing**: Terminal closes automatically after updates complete
- **Systemd Integration**: Easy setup with provided systemd service file
- **Terminal Detection**: Automatically finds available terminal emulators

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
   sudo systemctl daemon-reload
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

### Changing the Auto-close Timeout

By default, the terminal window closes 5 seconds after updates complete. To change this duration, modify the `sleep 5` line in the script.

### Terminal Emulator Priority

The script checks for terminal emulators in this order:
- alacritty
- konsole
- gnome-terminal
- xfce4-terminal
- mate-terminal
- terminator
- kitty
- urxvt
- xterm

To change the priority, modify the `terminals` vector in the `find_terminal_emulator()` function.

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

## Troubleshooting

### Terminal doesn't appear

If the terminal window doesn't appear at startup:
1. Check if your display server is running (X11/Wayland)
2. Verify the service is running: `systemctl status arch-auto-update.service`
3. Check for errors in the journal: `journalctl -u arch-auto-update.service`
4. Make sure a supported terminal emulator is installed

### Alternative Desktop Entry Method

If the systemd service approach doesn't work reliably for showing a terminal window, you can use the included desktop entry file instead:

1. Disable the systemd service:
   ```bash
   sudo systemctl disable arch-auto-update.service
   ```

2. Create an autostart directory if it doesn't exist:
   ```bash
   mkdir -p ~/.config/autostart
   ```

3. Copy the included desktop entry file to your autostart directory:
   ```bash
   cp arch-auto-update.desktop ~/.config/autostart/
   ```

4. Edit the desktop entry file to use your preferred terminal emulator:
   ```bash
   nano ~/.config/autostart/arch-auto-update.desktop
   ```
   
   Replace `TERMINAL_NAME` in the file with your terminal emulator (`gnome-terminal`, `konsole`, `xfce4-terminal`, `alacritty`, etc.)

5. Make the desktop entry executable:
   ```bash
   chmod +x ~/.config/autostart/arch-auto-update.desktop
   ```

This desktop entry approach is often more reliable for launching GUI applications at startup.

### Updates fail

If updates fail:
1. Try running the commands manually to check for errors:
   ```bash
   sudo pacman -Syu
   yay -Syu
   ```
2. Verify internet connectivity
3. Check for sufficient disk space

## FAQ

**Q: Is it safe to automatically update Arch Linux?**
A: While generally safe, automatic updates without supervision carry some risk. Consider implementing snapshot mechanisms before updates or checking the journal after updates for any issues.

**Q: Will this work with Wayland?**
A: Yes, but you may need to adjust the `DISPLAY` and `XAUTHORITY` environment variables in the service file.

**Q: Does this work with display managers other than GDM/SDDM?**
A: Yes, the script should work with any display manager as long as a graphical environment is available.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
