# Implementation Plan: bow (Universal Package Facade)

## Goal
Build a high-performance, zero-dependency Rust CLI named `bow` that standardizes package management. Start with an Arch Linux MVP (Pacman + AUR) while ensuring a provider-agnostic core.

## Standards & Constraints
- **Language**: Rust (Stable).
- **XDG Compliance**: Strictly use standard paths via the `directories` crate.
- **Modularity**: Use a `Trait` pattern for "Providers" (Arch, Debian, etc.).
- **Zero Bloat / High UX**:
    - Avoid heavy, unnecessary dependencies, but *invest* in crates that significantly improve UX (e.g., `indicatif` for spinners/progress, `comfy-table` for structured output).
    - Implement a `clean` command that prunes orphans and clears caches.
- **AUR Strategy**: Auto-detect `yay` or `paru`; if neither exists, fall back to native `pacman`.

## UX Enhancements (Nala-Inspired)
To achieve a "Nala-like" premium user experience:
- **Rich Terminal Output**: Use `comfy-table` or `tabled` to format search results and transaction summaries (Install/Upgrade/Remove/Size) into beautiful, readable tables instead of raw package manager text dumps.
- **Beautiful Error Reporting**: Use `miette` or `color-eyre` instead of `anyhow` for helpful, stylized error messages with context.
- **Interactive Prompts**: Use `inquire` or `dialoguer` for confirmation prompts, giving a polished interactive feel rather than a basic `(y/N)` read.
- **Spinners & Progress**: Use `indicatif` to show visually pleasing loading states during searches or dependency resolution.
- **Color & Styling**: Use `crossterm` or `owo-colors` to add vibrant, consistent color coding (e.g., Green for additions, Red for removals, Blue for info).

## Command Facade
- `s | search`  -> `yay -Ss` / `pacman -Ss`
- `i | install` -> `pacman -S --needed` (with AUR fallback)
- `u | update`  -> `yay -Syu` / `pacman -Syu`
- `c | clean`   -> Prune orphans and clear manager cache.
