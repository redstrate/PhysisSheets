#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OpenContent {
exd: EXD,
exh: EXH,
}
impl OpenContent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OpenContent").unwrap();let exd = game_data.read_excel_sheet("OpenContent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OpenContentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OpenContentRow { columns: row.columns.clone() }
}
}
pub struct OpenContentRow {
columns: Vec<ColumnData>,
}
impl OpenContentRow {
pub fn OpenContentData(&self) -> &ColumnData {
&self.columns[0]
}
}
