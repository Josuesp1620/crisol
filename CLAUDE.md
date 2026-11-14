# Crisol: rules for working on this repository

Crisol is a visual form designer for Rust, in the spirit of Windows Forms, built on GPUI.
The plan (architecture, phases, decisions) is in `docs/plan.md`.

## Architecture

- `crates/crisol-model`: the form document and its `.crisol` file format. No UI.
- `crates/crisol-catalog`: every control, its properties, events, defaults and design rules. No UI.
- `crates/crisol-commands`: reversible commands, transactions and undo history. No UI.
- `crates/crisol-engine`: designer logic (selection, hit testing, dragging, snapping). No UI.
- `crates/crisol-codegen`: code generators (Rust for GPUI first). No UI.
- `crates/crisol-ui`: Crisol's own GPUI components, theme tokens and icons.
- `crates/crisol-render`: draws a document with GPUI (designer canvas and preview).
- `apps/crisol-designer`: the designer application (`crisol.exe`).
- `apps/crisol-cli`: command line (`crisol-cli check`, `crisol-cli generate`).

Only `crisol-ui`, `crisol-render` and `crisol-designer` may depend on `gpui`. Business rules
live in the crates without UI; the interface draws what they decide.

## Code rules

- Object oriented: logic lives in methods of structs, enums and traits. No free functions,
  except `main` and test functions.
- Single responsibility at every level: one public type per file, named after it
  (`form_document.rs` holds `FormDocument`); one job per type, per method, per module.
- Small files and small methods. Split a type before it grows a second job.
- Descriptive, precise names. No identifiers of one, two or three letters, closure
  parameters included (`|node|`, never `|n|`).
- Prefer plain loops and methods over chains of iterator adapters and closures; a single,
  obvious adapter (`map`, `filter`) is fine. GPUI listeners are closures by design.
- No loose constants. Tunable values live in one settings type per layer:
  `FormatSettings` (model), `ThemeTokens` (ui), `DesignerSettings` (designer),
  `CodegenSettings` (codegen). Visible texts live in `DesignerTexts`.
- One error enum per crate, with `thiserror`, in `error.rs`.
- Modules are folders with a `mod.rs` that only declares and re-exports.
- Comments only when the reason is not obvious from the code. `///` on public types,
  short. No commented-out code, no restating what the code says.
- Everything in English: code, names, examples, comments, commit messages and visible texts.
- Every rule comes with a test in the crate's `tests/` folder, one file per feature.
- Never copy code from other projects. Slint, FigCraft and others are references for ideas only.
- Vet every new dependency (age, downloads, maintainers, advisories) before adding it.

## Toolchain

Everything runs in Docker; nothing is installed on the host. Use the Makefile:
`make images`, `make test`, `make test-all`, `make lint`, `make format`,
`make windows-check`, `make windows-build`.

Windows is the first platform. Release builds compile their HLSL shaders with `fxc.exe`, so
they are built on GitHub Actions (`windows-latest`), never cross-compiled.

## Commits, versions and releases

- One logical change per commit, message in the imperative, saying what and why.
- The author is the repository owner only. Never add `Co-Authored-By` lines or any AI
  attribution to commits, pull requests or releases.
- Check the staged file list before every commit; never commit secrets or `.cache/`.
- Versions follow Semantic Versioning, tags are `vX.Y.Z` (release candidates `vX.Y.Z-rc.N`).
- User-visible changes get an entry in `CHANGELOG.md` under `Unreleased`.
- To release: `make version VERSION=x.y.z`, move the `Unreleased` entries under the new
  version in `CHANGELOG.md`, commit, then push the tag. GitHub Actions builds the Windows
  zip and publishes the release.
