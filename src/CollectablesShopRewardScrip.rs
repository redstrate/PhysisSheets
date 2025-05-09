#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesShopRewardScrip {
exd: EXD,
exh: EXH,
}
impl CollectablesShopRewardScrip {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesShopRewardScrip").unwrap();let exd = game_data.read_excel_sheet("CollectablesShopRewardScrip", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesShopRewardScripRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesShopRewardScripRow { columns: row.columns.clone() }
}
}
pub struct CollectablesShopRewardScripRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopRewardScripRow {
pub fn Currency(&self) -> &ColumnData {
&self.columns[0]
}
pub fn LowReward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MidReward(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HighReward(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ExpRatioLow(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ExpRatioMid(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ExpRatioHigh(&self) -> &ColumnData {
&self.columns[6]
}
}
