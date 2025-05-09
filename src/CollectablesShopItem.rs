#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesShopItem {
exd: EXD,
exh: EXH,
}
impl CollectablesShopItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesShopItem").unwrap();let exd = game_data.read_excel_sheet("CollectablesShopItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesShopItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesShopItemRow { columns: row.columns.clone() }
}
}
pub struct CollectablesShopItemRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LevelMin(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LevelMax(&self) -> &ColumnData {
&self.columns[3]
}
pub fn CollectablesShopRefine(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CollectablesShopRewardScrip(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CollectablesShopItemGroup(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[8]
}
}
