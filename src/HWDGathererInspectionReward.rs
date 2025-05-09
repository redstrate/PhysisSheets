#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDGathererInspectionReward {
exd: EXD,
exh: EXH,
}
impl HWDGathererInspectionReward {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDGathererInspectionReward").unwrap();let exd = game_data.read_excel_sheet("HWDGathererInspectionReward", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDGathererInspectionRewardRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDGathererInspectionRewardRow { columns: row.columns.clone() }
}
}
pub struct HWDGathererInspectionRewardRow {
columns: Vec<ColumnData>,
}
impl HWDGathererInspectionRewardRow {
pub fn Scrips(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Points(&self) -> &ColumnData {
&self.columns[1]
}
}
