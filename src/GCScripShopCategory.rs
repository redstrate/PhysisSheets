#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GCScripShopCategory {
exd: EXD,
exh: EXH,
}
impl GCScripShopCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GCScripShopCategory").unwrap();let exd = game_data.read_excel_sheet("GCScripShopCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GCScripShopCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GCScripShopCategoryRow { columns: row.columns.clone() }
}
}
pub struct GCScripShopCategoryRow {
columns: Vec<ColumnData>,
}
impl GCScripShopCategoryRow {
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Tier(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SubCategory(&self) -> &ColumnData {
&self.columns[2]
}
}
