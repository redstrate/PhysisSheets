#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GCScripShopItem {
exd: EXD,
exh: EXH,
}
impl GCScripShopItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GCScripShopItem").unwrap();let exd = game_data.read_excel_sheet("GCScripShopItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GCScripShopItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GCScripShopItemRow { columns: row.columns.clone() }
}
}
pub struct GCScripShopItemRow {
columns: Vec<ColumnData>,
}
impl GCScripShopItemRow {
pub fn CostGCSeals(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredGrandCompanyRank(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[3]
}
}
