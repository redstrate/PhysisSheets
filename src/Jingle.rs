#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Jingle {
exd: EXD,
exh: EXH,
}
impl Jingle {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Jingle").unwrap();let exd = game_data.read_excel_sheet("Jingle", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> JingleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
JingleRow { columns: row.columns.clone() }
}
}
pub struct JingleRow {
columns: Vec<ColumnData>,
}
impl JingleRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
