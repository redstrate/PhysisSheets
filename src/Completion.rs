#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Completion {
exd: EXD,
exh: EXH,
}
impl Completion {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Completion").unwrap();let exd = game_data.read_excel_sheet("Completion", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompletionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompletionRow { columns: row.columns.clone() }
}
}
pub struct CompletionRow {
columns: Vec<ColumnData>,
}
impl CompletionRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GroupTitle(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LookupTable(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Group(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[4]
}
}
