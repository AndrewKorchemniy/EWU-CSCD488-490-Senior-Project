# Repo Layout

## Code

- Main Cargo file
    - [cargo](../Cargo.toml)
- Executables
    - [backend (server)](../backend)
    - [dbcli (tool)](../dbcli)
- Library's
    - [common (lib)](../common)
    - [database (lib)](../database)
- Pages
    - [student page (web)](../studentpage)
    - [admin page (web)](../adminpage)

### folder layout

- **src** folder (all): where the source file are
  - Note: **tests** are in the same src files for rust
  - Note: rust file end in `rs`
- **Cargo.toml** file (all): The config of the project
  - Big things are: `name` and `dependencies`
- **dist** folder (web): final output page from trunk
- **target** folder (all-sometimes): output/building of the source code using cargo
- **index.html** file (web): trunk use as starting place to make the page
- **Truck.toml** file (web): config trunk
- **migrations** folder (database/diesel): used to make [schema.rs](../database/src/repository/schema.rs) file and to setup database

## Scripts

- Git
    - [GitHub (actions)](../.github)
    - [Ignore file (do not commit/push)](../.gitignore)
    - [GitHub (pages)](../_config.yml)
- Helper Scripts
    - [helper scripts](../helper_scripts)

## Docs

- [Doc folder](../Doc)
- [Main Readme](../README.md)

### folder layout

- **README.md** file: Starting page (often Table of contents)

## From IDE's

- [Jet Brains IDE's](../.idea)
- [Visual Studio Code](../.vscode)