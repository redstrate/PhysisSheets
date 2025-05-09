#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EmjCostumeData {
exd: EXD,
exh: EXH,
}
impl EmjCostumeData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EmjCostumeData").unwrap();let exd = game_data.read_excel_sheet("EmjCostumeData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EmjCostumeDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EmjCostumeDataRow { columns: row.columns.clone() }
}
}
pub struct EmjCostumeDataRow {
columns: Vec<ColumnData>,
}
impl EmjCostumeDataRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
