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

## Interactive Update/Install Flow (Nala-Inspired)
Package managers like `pacman` and `yay` prompt the user for input mid-execution, which breaks if we try to hide their output behind a spinner. To solve this and provide an escape hatch to native output:

1. **Pre-flight Check**: Run the package manager with arguments to get a list of pending transactions (e.g., `yay -Qu` or `pacman -Qu`).
2. **Custom Summary**: Parse this list and present it in a beautiful `comfy-table` (showing packages, old version -> new version).
3. **Custom Prompt**: Use `inquire::Confirm` to ask the user if they want to proceed, rather than relying on `pacman`'s raw prompt.
4. **Non-Interactive Execution**: Run `yay -Syu --noconfirm` as a piped subprocess.
5. **Spinner & Escape Hatch**: 
    - Show an `indicatif` spinner ("Updating system... Press 'v' for raw output").
    - Use `crossterm` in raw mode to listen for keypresses non-blockingly while reading the subprocess output in a background thread.
    - If the user presses `v`, we clear the spinner and seamlessly pipe the real-time package manager output straight to the terminal so they can see exactly what is happening.
