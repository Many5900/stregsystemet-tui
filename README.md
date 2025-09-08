# Stregsystemet TUI

A modern, feature-rich Terminal User Interface (TUI) for the Stregsystemet beverage purchasing system.

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.1.0-blue.svg)](https://github.com/Many5900/stregsystemet-tui)

## Overview

This TUI application provides an efficient way to interact with the Stregsystemet beverage purchasing system, without having to leave your terminal 😉. It offers vim-like navigation, search capabilities, and integrated parking registration for parking at AAU (Aalborg University) Zone 4688. (You might also find [AAU Parking Scheduler](https://github.com/Many5900/aau-parking-scheduler) interesting)

![Demo](demo.gif)

## ⚠️ Important Warning

**ALWAYS verify your parking registrations!**
- Check your SMS messages for confirmation from Apcoa

**The author takes no responsibility for:**
- Failed parking registrations
- Parking fines due to unsuccessful registrations
- System outages or API failures

## Requirements

Before setting up the application, make sure you have the following installed:

**Rust**: Installation instructions can be found at [https://rustup.rs/](https://rustup.rs/) <br>
**Cargo**: Comes with Rust installation <br>

## Installation

1. **Install the latest version:**
   ```bash
   # Install the latest version
   cargo install stregsystemet-tui

   # The binary will be available as 'stui' in your PATH
   stui
   ```

2. **Update to latest version:**
   ```bash
   cargo install stregsystemet-tui --force
   ```

3. **Uninstall:**
   ```bash
   cargo uninstall stregsystemet-tui
   ```

## Running the Application

Once installed, start the application by typing `stui` in your terminal:

```bash
stui
```

## Configuration

The application automatically creates a configuration file at `~/.config/.stregsystemet.toml` when you first run it. You don't need to worry about this file, the application works perfectly without any manual configuration.

However, if you want to customize settings, you can edit the file:

```toml
username = "your_username"
room_id = 10
phone_number = "12345678"      # Optional: saved from parking registration
license_plate = "AB12345"    # Optional: saved from parking registration
```

## Basic Navigation

The application displays helpful key bindings throughout the interface. These hints indicate which keys are available based on your current mode and context.

The following are the primary key bindings:

### **Product List**
| Key | Action |
|-----|--------|
| `j/k` or `↓/↑` | Navigate up/down |
| `gg` | Go to top |
| `G` | Go to bottom |
| `[num]j/k` | Jump N lines (vim-style) |
| `Enter` | Purchase selected product |
| `/` or `s` | Search products |

### **Purchase Flow**
| Key | Action |
|-----|--------|
| `Enter` | Open purchase modal |
| `+/-` or `←/→` | Adjust quantity |
| `y` | Confirm purchase |
| `n/Esc` | Cancel |

### **Search**
| Key | Action |
|-----|--------|
| `/` or `s` | Open search modal |
| `Ctrl+n/p` | Navigate results |
| `Enter` | Select product |
| `Esc` | Close search |

### **Parking Registration**
| Key | Action |
|-----|--------|
| `p` | Open parking modal |
| `Tab` | Switch between fields |
| `Enter` | Confirm details |
| `y` | Register parking |
| `n` | Cancel |

### **Other Commands**
| Key | Action |
|-----|--------|
| `u` | Change username |
| `q` | Quit application |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for terminal UI
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling
- Integrates with the [Stregsystemet](https://github.com/f-klubben/stregsystemet) API

---

**Made with ❤️ in Rust**
