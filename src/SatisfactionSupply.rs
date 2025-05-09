#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SatisfactionSupply {
exd: EXD,
exh: EXH,
}
impl SatisfactionSupply {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SatisfactionSupply").unwrap();let exd = game_data.read_excel_sheet("SatisfactionSupply", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SatisfactionSupplyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SatisfactionSupplyRow { columns: row.columns.clone() }
}
}
pub struct SatisfactionSupplyRow {
columns: Vec<ColumnData>,
}
impl SatisfactionSupplyRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CollectabilityLow(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CollectabilityMid(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CollectabilityHigh(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn FishingSpotId(&self) -> &ColumnData {
&self.columns[5]
}
pub fn SpearFishingSpotId(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ProbabilityPercent(&self) -> &ColumnData {
&self.columns[8]
}
pub fn IsBonus(&self) -> &ColumnData {
&self.columns[9]
}
}
