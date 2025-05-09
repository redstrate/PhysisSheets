#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringPointTransient {
exd: EXD,
exh: EXH,
}
impl GatheringPointTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringPointTransient").unwrap();let exd = game_data.read_excel_sheet("GatheringPointTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringPointTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringPointTransientRow { columns: row.columns.clone() }
}
}
pub struct GatheringPointTransientRow {
columns: Vec<ColumnData>,
}
impl GatheringPointTransientRow {
pub fn GatheringRarePopTimeTable(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EphemeralStartTime(&self) -> &ColumnData {
&self.columns[1]
}
pub fn EphemeralEndTime(&self) -> &ColumnData {
&self.columns[2]
}
}
