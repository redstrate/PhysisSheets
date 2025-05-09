#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCActivityCategory {
exd: EXD,
exh: EXH,
}
impl FCActivityCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCActivityCategory").unwrap();let exd = game_data.read_excel_sheet("FCActivityCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCActivityCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCActivityCategoryRow { columns: row.columns.clone() }
}
}
pub struct FCActivityCategoryRow {
columns: Vec<ColumnData>,
}
impl FCActivityCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Priority(&self) -> &ColumnData {
&self.columns[1]
}
}
