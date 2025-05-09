#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringRarePopTimeTable {
exd: EXD,
exh: EXH,
}
impl GatheringRarePopTimeTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringRarePopTimeTable").unwrap();let exd = game_data.read_excel_sheet("GatheringRarePopTimeTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringRarePopTimeTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringRarePopTimeTableRow { columns: row.columns.clone() }
}
}
pub struct GatheringRarePopTimeTableRow {
columns: Vec<ColumnData>,
}
impl GatheringRarePopTimeTableRow {
pub fn StartTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Duration(&self) -> &ColumnData {
&self.columns[1]
}
}
