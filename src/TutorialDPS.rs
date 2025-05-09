#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TutorialDPS {
exd: EXD,
exh: EXH,
}
impl TutorialDPS {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TutorialDPS").unwrap();let exd = game_data.read_excel_sheet("TutorialDPS", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TutorialDPSRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TutorialDPSRow { columns: row.columns.clone() }
}
}
pub struct TutorialDPSRow {
columns: Vec<ColumnData>,
}
impl TutorialDPSRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[1]
}
}
