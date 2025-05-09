#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HouseRetainerPose {
exd: EXD,
exh: EXH,
}
impl HouseRetainerPose {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HouseRetainerPose").unwrap();let exd = game_data.read_excel_sheet("HouseRetainerPose", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HouseRetainerPoseRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HouseRetainerPoseRow { columns: row.columns.clone() }
}
}
pub struct HouseRetainerPoseRow {
columns: Vec<ColumnData>,
}
impl HouseRetainerPoseRow {
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[0]
}
}
