#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesShop {
exd: EXD,
exh: EXH,
}
impl CollectablesShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesShop").unwrap();let exd = game_data.read_excel_sheet("CollectablesShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesShopRow { columns: row.columns.clone() }
}
}
pub struct CollectablesShopRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ShopItems(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RewardType(&self) -> &ColumnData {
&self.columns[3]
}
}
