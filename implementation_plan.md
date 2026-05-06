# Implementation Plan: bow (Universal Package Facade)

## Goal
Build a high-performance, zero-dependency Rust CLI named `bow` that standardizes package management. Start with an Arch Linux MVP (Pacman + AUR) while ensuring a provider-agnostic core.

## Standards & Constraints
- **Language**: Rust (Stable).
- **XDG Compliance**: Strictly use standard paths via the `directories` crate.
- **Modularity**: Use a `Trait` pattern for "Providers" (Arch, Debian, etc.).
- **Zero Bloat**:
    - Avoid heavy dependencies.
    - Implement a `clean` command that prunes orphans and clears caches.
- **AUR Strategy**: Auto-detect `yay` or `paru`; if neither exists, fall back to native `pacman`.

## Command Facade
- `s | search`  -> `yay -Ss` / `pacman -Ss`
- `i | install` -> `pacman -S --needed` (with AUR fallback)
- `u | update`  -> `yay -Syu` / `pacman -Syu`
- `c | clean`   -> Prune orphans and clear manager cache.
