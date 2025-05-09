#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OrchestrionUiparam {
exd: EXD,
exh: EXH,
}
impl OrchestrionUiparam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OrchestrionUiparam").unwrap();let exd = game_data.read_excel_sheet("OrchestrionUiparam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrchestrionUiparamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OrchestrionUiparamRow { columns: row.columns.clone() }
}
}
pub struct OrchestrionUiparamRow {
columns: Vec<ColumnData>,
}
impl OrchestrionUiparamRow {
pub fn Order(&self) -> &ColumnData {
&self.columns[0]
}
pub fn OrchestrionCategory(&self) -> &ColumnData {
&self.columns[1]
}
}
