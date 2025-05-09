#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PerformGroup {
exd: EXD,
exh: EXH,
}
impl PerformGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PerformGroup").unwrap();let exd = game_data.read_excel_sheet("PerformGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PerformGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PerformGroupRow { columns: row.columns.clone() }
}
}
pub struct PerformGroupRow {
columns: Vec<ColumnData>,
}
impl PerformGroupRow {
pub fn Perform(&self) -> &ColumnData {
&self.columns[0]
}
}
