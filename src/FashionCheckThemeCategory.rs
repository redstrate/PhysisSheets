#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FashionCheckThemeCategory {
exd: EXD,
exh: EXH,
}
impl FashionCheckThemeCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FashionCheckThemeCategory").unwrap();let exd = game_data.read_excel_sheet("FashionCheckThemeCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FashionCheckThemeCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FashionCheckThemeCategoryRow { columns: row.columns.clone() }
}
}
pub struct FashionCheckThemeCategoryRow {
columns: Vec<ColumnData>,
}
impl FashionCheckThemeCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
