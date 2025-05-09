#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GuidePageString {
exd: EXD,
exh: EXH,
}
impl GuidePageString {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GuidePageString").unwrap();let exd = game_data.read_excel_sheet("GuidePageString", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GuidePageStringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GuidePageStringRow { columns: row.columns.clone() }
}
}
pub struct GuidePageStringRow {
columns: Vec<ColumnData>,
}
impl GuidePageStringRow {
pub fn String(&self) -> &ColumnData {
&self.columns[0]
}
}
