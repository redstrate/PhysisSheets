#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonBan {
exd: EXD,
exh: EXH,
}
impl DeepDungeonBan {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonBan").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonBan", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonBanRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonBanRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonBanRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonBanRow {
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
