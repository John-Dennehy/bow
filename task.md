- [x] Initialize Cargo project and add core crates (`clap`, `directories`).
- [x] Add UX crates (`color-eyre`/`miette`, `indicatif`, `comfy-table`, `inquire`, `owo-colors`).
- [x] Implement XDG path resolution for config/state.
- [x] Create `PackageProvider` trait in `src/provider.rs` with methods returning structured data.
- [x] Implement `ArchProvider` (handle `sudo`, `pacman`, and AUR helper detection).
- [x] Intercept and parse package manager output to render rich tables and summaries (Nala-style).
- [x] Build the CLI routing logic (matching `s`, `i`, `u`, `c` and full words).
- [x] Implement the `clean` module (orphan pruning logic).
- [x] Generate the final portfolio-ready README.md.

## Interactive Update Feature
- [x] Add `crossterm` for background keypress listening.
- [x] Implement `yay -Qu` parsing to show pending updates.
- [x] Add `inquire` prompt for update confirmation.
- [x] Implement `indicatif` spinner with `crossterm` escape hatch.
- [x] Stream stdout/stderr seamlessly when user presses 'v'.
