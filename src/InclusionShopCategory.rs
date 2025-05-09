#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct InclusionShopCategory {
exd: EXD,
exh: EXH,
}
impl InclusionShopCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("InclusionShopCategory").unwrap();let exd = game_data.read_excel_sheet("InclusionShopCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> InclusionShopCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
InclusionShopCategoryRow { columns: row.columns.clone() }
}
}
pub struct InclusionShopCategoryRow {
columns: Vec<ColumnData>,
}
impl InclusionShopCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn InclusionShopSeries(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[2]
}
}
