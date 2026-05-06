# 🎀 bow

**Put a bow on your package management.**

## 🚀 The Vision

I could never remember the different commands for package managers when I switched between Arch, Debian, Fedora, and macOS. So I made `bow`, a small cross-platform CLI front-end that abstracts the friction of distro-hopping. 

Inspired by `nala` on Debian, `bow` doesn't just wrap package managers; it makes them beautiful. It is built in Rust, features rich terminal UIs, and follows the XDG Base Directory Specification for configuration and state. (This is my first Rust project!)

## 🛠 Features

- **Unified Syntax**: Memorize five letters: `s`, `i`, `r`, `u`, `c`.
- **Beautiful UX**: Rich tables (`comfy-table`), vibrant colors (`owo-colors`), and clean error reporting (`miette`).
- **AUR Integration**: Seamlessly bridges the gap between official repos and the AUR (auto-detects `yay` and `paru`).
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
bow r <package>
```

Uninstalls a package from the system.

### Update

```bash
bow u
```

Updates the system.

### Clean

```bash
bow c
```

Cleans the system of orphan packages.

## 🔮 Future Plans

- [ ] Automatic AUR helper detection (yay, paru).
- [ ] Add support for Debian/Ubuntu (`apt`) and Fedora (`dnf`).
- [ ] Add support for macOS (`brew`).
