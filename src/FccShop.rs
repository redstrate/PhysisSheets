#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FccShop {
exd: EXD,
exh: EXH,
}
impl FccShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FccShop").unwrap();let exd = game_data.read_excel_sheet("FccShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FccShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FccShopRow { columns: row.columns.clone() }
}
}
pub struct FccShopRow {
columns: Vec<ColumnData>,
}
impl FccShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemData(&self) -> &ColumnData {
&self.columns[1]
}
}
