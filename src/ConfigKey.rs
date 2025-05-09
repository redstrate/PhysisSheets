#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ConfigKey {
exd: EXD,
exh: EXH,
}
impl ConfigKey {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ConfigKey").unwrap();let exd = game_data.read_excel_sheet("ConfigKey", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ConfigKeyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ConfigKeyRow { columns: row.columns.clone() }
}
}
pub struct ConfigKeyRow {
columns: Vec<ColumnData>,
}
impl ConfigKeyRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Label(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Param(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Platform(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[5]
}
pub fn BacklightColor(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Required(&self) -> &ColumnData {
&self.columns[7]
}
}
