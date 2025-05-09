#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CabinetSubCategory {
exd: EXD,
exh: EXH,
}
impl CabinetSubCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CabinetSubCategory").unwrap();let exd = game_data.read_excel_sheet("CabinetSubCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CabinetSubCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CabinetSubCategoryRow { columns: row.columns.clone() }
}
}
pub struct CabinetSubCategoryRow {
columns: Vec<ColumnData>,
}
impl CabinetSubCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MenuOrder(&self) -> &ColumnData {
&self.columns[1]
}
}
