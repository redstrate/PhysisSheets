#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OrchestrionPath {
exd: EXD,
exh: EXH,
}
impl OrchestrionPath {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OrchestrionPath").unwrap();let exd = game_data.read_excel_sheet("OrchestrionPath", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrchestrionPathRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OrchestrionPathRow { columns: row.columns.clone() }
}
}
pub struct OrchestrionPathRow {
columns: Vec<ColumnData>,
}
impl OrchestrionPathRow {
pub fn File(&self) -> &ColumnData {
&self.columns[0]
}
}
