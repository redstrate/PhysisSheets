#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIGathering {
exd: EXD,
exh: EXH,
}
impl MJIGathering {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIGathering").unwrap();let exd = game_data.read_excel_sheet("MJIGathering", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIGatheringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIGatheringRow { columns: row.columns.clone() }
}
}
pub struct MJIGatheringRow {
columns: Vec<ColumnData>,
}
impl MJIGatheringRow {
pub fn GatheringObject(&self) -> &ColumnData {
&self.columns[0]
}
}
