#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSEmergencyInfoText {
exd: EXD,
exh: EXH,
}
impl WKSEmergencyInfoText {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSEmergencyInfoText").unwrap();let exd = game_data.read_excel_sheet("WKSEmergencyInfoText", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSEmergencyInfoTextRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSEmergencyInfoTextRow { columns: row.columns.clone() }
}
}
pub struct WKSEmergencyInfoTextRow {
columns: Vec<ColumnData>,
}
impl WKSEmergencyInfoTextRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
