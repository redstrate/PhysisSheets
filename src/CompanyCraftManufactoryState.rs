#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftManufactoryState {
exd: EXD,
exh: EXH,
}
impl CompanyCraftManufactoryState {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftManufactoryState").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftManufactoryState", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftManufactoryStateRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftManufactoryStateRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftManufactoryStateRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftManufactoryStateRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
