# Icarus

This is an auto-generated library for Rust, based on [EXDSchema](https://github.com/xivdev/EXDSchema/) and [Physis](https://github.com/redstrate/physis). The generator is located [here](https://github.com/redstrate/EXDGen).

**Applicable Game Version**: `2025.06.10.0000.0000`.

**NOTE:** This is still a WIP, the full schema is not supported yet nor is the API finalized.

## Usage

Include it in your `Cargo.toml`:

```toml
[dependencies]
icarus = { git = "https://github.com/redstrate/Icarus", branch = "2025.06.10.0000.0000", default-features = false }
```

By default, **all sheets are included**. For most situations you will not want that[^1], hence `no-default-features` above. Instead, include the sheets you actually need:

```toml
[dependencies]
icarus = { git = "https://github.com/redstrate/Icarus", branch= "2025.06.10.0000.0000", features = ["Item"], default-features = false }
```

Then include the sheet and use it!

```rust
let item_sheet = ItemSheet::read_from(&mut game_data, Language::English);
let item_row = item_sheet.get_row(1);

println!("{:#?}", item_row.Singular());
```

[^1]: It also increases your compile time by several seconds!
