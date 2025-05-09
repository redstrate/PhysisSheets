#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WebURL {
exd: EXD,
exh: EXH,
}
impl WebURL {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WebURL").unwrap();let exd = game_data.read_excel_sheet("WebURL", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WebURLRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WebURLRow { columns: row.columns.clone() }
}
}
pub struct WebURLRow {
columns: Vec<ColumnData>,
}
impl WebURLRow {
pub fn URL(&self) -> &ColumnData {
&self.columns[0]
}
}
