#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingMerchantPose {
exd: EXD,
exh: EXH,
}
impl HousingMerchantPose {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingMerchantPose").unwrap();let exd = game_data.read_excel_sheet("HousingMerchantPose", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingMerchantPoseRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingMerchantPoseRow { columns: row.columns.clone() }
}
}
pub struct HousingMerchantPoseRow {
columns: Vec<ColumnData>,
}
impl HousingMerchantPoseRow {
pub fn Pose(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[1]
}
}
