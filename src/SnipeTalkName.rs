#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SnipeTalkName {
exd: EXD,
exh: EXH,
}
impl SnipeTalkName {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SnipeTalkName").unwrap();let exd = game_data.read_excel_sheet("SnipeTalkName", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SnipeTalkNameRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SnipeTalkNameRow { columns: row.columns.clone() }
}
}
pub struct SnipeTalkNameRow {
columns: Vec<ColumnData>,
}
impl SnipeTalkNameRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
