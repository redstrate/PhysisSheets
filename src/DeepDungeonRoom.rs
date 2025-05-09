#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonRoom {
exd: EXD,
exh: EXH,
}
impl DeepDungeonRoom {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonRoom").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonRoom", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonRoomRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonRoomRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonRoomRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonRoomRow {
pub fn Level(&self) -> &ColumnData {
&self.columns[0]
}
}
