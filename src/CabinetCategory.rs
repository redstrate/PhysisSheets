#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CabinetCategory {
exd: EXD,
exh: EXH,
}
impl CabinetCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CabinetCategory").unwrap();let exd = game_data.read_excel_sheet("CabinetCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CabinetCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CabinetCategoryRow { columns: row.columns.clone() }
}
}
pub struct CabinetCategoryRow {
columns: Vec<ColumnData>,
}
impl CabinetCategoryRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MenuOrder(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HideOrder(&self) -> &ColumnData {
&self.columns[3]
}
}
