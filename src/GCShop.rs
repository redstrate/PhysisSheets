#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GCShop {
exd: EXD,
exh: EXH,
}
impl GCShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GCShop").unwrap();let exd = game_data.read_excel_sheet("GCShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GCShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GCShopRow { columns: row.columns.clone() }
}
}
pub struct GCShopRow {
columns: Vec<ColumnData>,
}
impl GCShopRow {
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[0]
}
}
