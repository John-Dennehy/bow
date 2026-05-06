# 🎀 bow

**Put a bow on your package management.**

## 🚀 The Vision

I could never remember thes different commands for package managers when I switched between Arch, Debian, Fedora, and macOS. So I made `bow`, a small cross-platform CLI front-end that abstracts the friction of distro-hopping. It is built in Rust and follows the XDG Base Directory Specification for configuration and state. (This is my first Rust project!)

## 🛠 Features

- **Unified Syntax**: Memorize four letters: `s`, `i`, `u`, `c`.
- **AUR Integration**: Seamlessly bridges the gap between official repos and the AUR.
- **XDG Native**: No clutter. Strictly adheres to system standards.
- **Space Optimization**: Aggressive cleanup routines to keep your disk lean.

## 📦 Commands

### Search

```bash
bow s <package>
```

Searches for a package in the official repositories and the AUR.

### Install

```bash
bow i <package>
```

Installs a package from the official repositories or the AUR.

### Uninstall

```bash
bow u <package>
```

Uninstalls a package from the system.

### Clean

```bash
bow c
```

Cleans the system of orphan packages.

## 🔮 Future Plans

- [ ] Automatic AUR helper detection (yay, paru).
- [ ] Add support for Debian/Ubuntu (`apt`) and Fedora (`dnf`).
- [ ] Add support for macOS (`brew`).
