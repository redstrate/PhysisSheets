#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonStatus {
exd: EXD,
exh: EXH,
}
impl DeepDungeonStatus {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonStatus").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonStatus", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonStatusRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonStatusRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonStatusRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonStatusRow {
pub fn ScreenImage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn LogMessage(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
}
