# Changelog

All notable changes to Crisol are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versions follow
[Semantic Versioning](https://semver.org/). Release candidates are tagged `vX.Y.Z-rc.N`.

## [Unreleased]

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
