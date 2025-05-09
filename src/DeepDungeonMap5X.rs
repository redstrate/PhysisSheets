#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonMap5X {
exd: EXD,
exh: EXH,
}
impl DeepDungeonMap5X {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonMap5X").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonMap5X", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonMap5XRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonMap5XRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonMap5XRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonMap5XRow {
pub fn DeepDungeonRoom(&self) -> &ColumnData {
&self.columns[0]
}
}
