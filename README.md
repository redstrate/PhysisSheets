# Physis Sheets

This is an auto-generated library for Rust, based on [EXDSchema] and [Physis]. The generator is located [here].

**Applicable Game Version**: `2025.04.16.0000.0000`.

## Usage

Include it in your `Cargo.toml`:

```toml
[dependencies]
physis-sheets = { git = "https://github.com/redstrate/PhysisSheets", no-default-features = true }
```

By default, **all sheets are included**. For most situations you will not want that[^1], hence `no-default-features` above. Instead, include the sheets you actually need:

```toml
[dependencies]
physis-sheets = { git = "https://github.com/redstrate/PhysisSheets", features = ["Item"], no-default-features = true }
```

Then include the sheet and use it!

```rust
let item_sheet = Item::read_from(&mut game_data, Language::English);
let item_row = item_sheet.get_row(1);

println!("{:#?}", item_row.Singular());
```

[^1]: It also increases your compile time by several seconds!
