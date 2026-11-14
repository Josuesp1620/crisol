# Changelog

All notable changes to Crisol are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versions follow
[Semantic Versioning](https://semver.org/). Release candidates are tagged `vX.Y.Z-rc.N`.

## [Unreleased]

## [0.2.0] - 2026-11-14

### Added
- A Windows installer (`crisol-vX.Y.Z-setup.exe`): Start menu and desktop shortcuts, an
  uninstaller, and `.crisol` and `.crisolproj` files open in Crisol with a double-click.
- Projects: the start page, the New project dialog (desktop application, data form, dialog
  or empty template; name, location and theme), the `.crisolproj` file, recent projects,
  Add form (Ctrl+Shift+A), a project tree in the explorer and Save all (Ctrl+Shift+S).
- Every new project is a Cargo project that builds and runs as it is. Build (Ctrl+Shift+B)
  and Run project (Ctrl+F5) use cargo when Rust is installed, with the output in the
  Output panel.
- Saving a form writes its Rust code and keeps `forms/mod.rs` current.
- Code tabs: View code (F7), Designer (Shift+F7); double-clicking a control connects its main
  event and opens your file at the method, as in Windows Forms; open any file in your editor.
- The form resizes from its right and bottom edges on the canvas.
- Copy, paste and duplicate controls (Ctrl+C, Ctrl+V, Ctrl+D), move them with the arrow keys
  (Shift for 8 pixels), Esc selects the container.
- Align left, center, right, top and bottom, make same size and space evenly (Format menu and
  tool bar) for controls on an absolute panel.
- Command palette (Ctrl+Shift+P), toolbox search, property search, alphabetical order,
  folding categories and groups, the list of all controls in the property grid, clickable
  errors, a foldable bottom panel, tooltips and closing tabs with a save prompt.

### Changed
- Generated files are named after the form file: `customer.crisol` gives
  `customer.generated.rs` and `customer_events.rs`.
- The pointer shows a hand over controls and buttons, a closed hand while dragging and
  arrows on resize handles.

### Fixed
- No console window opens behind the designer on Windows.

## [0.1.0] - 2026-11-14

### Added
- The designer, laid out like the Windows Forms designer in Visual Studio: toolbox, canvas
  with the form window, error list, output and history, project explorer and property grid
  with descriptions, plus menus, tool bar and status bar. Light and dark themes.
- Canvas editing: select, multi-select, move, resize, reorder inside stacks and grids, drop
  from the toolbox, smart guides, delete, and undo/redo for every change.
- Property and event editing in place: text, numbers, lists, yes/no values, choices, control
  names and the form's title, type name and size.
- The `.crisol` format: canonical JSON that writes the same bytes for the same form.
- Validation with errors and portability and accessibility warnings.
- Rust code generation for GPUI: the form struct, its events trait and the render tree,
  plus a handlers file that Crisol only ever extends.
- Run: the form in its own window with working controls.
- `crisol-cli check` and `crisol-cli generate`.
- `crisol-ui`, Crisol's own components: Label, Button, TextBox, CheckBox, RadioButton,
  ComboBox, Stack, Grid, Group and an anchored Panel.
- The customer example, built from generated code.
