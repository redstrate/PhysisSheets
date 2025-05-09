#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WebGuidance {
exd: EXD,
exh: EXH,
}
impl WebGuidance {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WebGuidance").unwrap();let exd = game_data.read_excel_sheet("WebGuidance", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WebGuidanceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WebGuidanceRow { columns: row.columns.clone() }
}
}
pub struct WebGuidanceRow {
columns: Vec<ColumnData>,
}
impl WebGuidanceRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Url(&self) -> &ColumnData {
&self.columns[4]
}
}
