#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CycleTime {
exd: EXD,
exh: EXH,
}
impl CycleTime {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CycleTime").unwrap();let exd = game_data.read_excel_sheet("CycleTime", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CycleTimeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CycleTimeRow { columns: row.columns.clone() }
}
}
pub struct CycleTimeRow {
columns: Vec<ColumnData>,
}
impl CycleTimeRow {
pub fn FirstCycle(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Cycle(&self) -> &ColumnData {
&self.columns[1]
}
}
