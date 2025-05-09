#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionCastVFX {
exd: EXD,
exh: EXH,
}
impl ActionCastVFX {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionCastVFX").unwrap();let exd = game_data.read_excel_sheet("ActionCastVFX", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionCastVFXRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionCastVFXRow { columns: row.columns.clone() }
}
}
pub struct ActionCastVFXRow {
columns: Vec<ColumnData>,
}
impl ActionCastVFXRow {
pub fn VFX(&self) -> &ColumnData {
&self.columns[0]
}
}
