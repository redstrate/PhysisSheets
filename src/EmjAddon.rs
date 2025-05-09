#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EmjAddon {
exd: EXD,
exh: EXH,
}
impl EmjAddon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EmjAddon").unwrap();let exd = game_data.read_excel_sheet("EmjAddon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EmjAddonRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EmjAddonRow { columns: row.columns.clone() }
}
}
pub struct EmjAddonRow {
columns: Vec<ColumnData>,
}
impl EmjAddonRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
