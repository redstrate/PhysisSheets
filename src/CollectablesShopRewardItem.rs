#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesShopRewardItem {
exd: EXD,
exh: EXH,
}
impl CollectablesShopRewardItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesShopRewardItem").unwrap();let exd = game_data.read_excel_sheet("CollectablesShopRewardItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesShopRewardItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesShopRewardItemRow { columns: row.columns.clone() }
}
}
pub struct CollectablesShopRewardItemRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopRewardItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RewardLow(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RewardMid(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RewardHigh(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[9]
}
}
