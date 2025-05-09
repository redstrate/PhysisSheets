#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ExVersion {
exd: EXD,
exh: EXH,
}
impl ExVersion {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ExVersion").unwrap();let exd = game_data.read_excel_sheet("ExVersion", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ExVersionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ExVersionRow { columns: row.columns.clone() }
}
}
pub struct ExVersionRow {
columns: Vec<ColumnData>,
}
impl ExVersionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MenuScreen(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AcceptJingle(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CompleteJingle(&self) -> &ColumnData {
&self.columns[3]
}
}
