#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MountAction {
exd: EXD,
exh: EXH,
}
impl MountAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MountAction").unwrap();let exd = game_data.read_excel_sheet("MountAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MountActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MountActionRow { columns: row.columns.clone() }
}
}
pub struct MountActionRow {
columns: Vec<ColumnData>,
}
impl MountActionRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
}
