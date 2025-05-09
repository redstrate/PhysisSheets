#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Tomestones {
exd: EXD,
exh: EXH,
}
impl Tomestones {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Tomestones").unwrap();let exd = game_data.read_excel_sheet("Tomestones", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TomestonesRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TomestonesRow { columns: row.columns.clone() }
}
}
pub struct TomestonesRow {
columns: Vec<ColumnData>,
}
impl TomestonesRow {
pub fn WeeklyLimit(&self) -> &ColumnData {
&self.columns[0]
}
}
