#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RecipeNotebookList {
exd: EXD,
exh: EXH,
}
impl RecipeNotebookList {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RecipeNotebookList").unwrap();let exd = game_data.read_excel_sheet("RecipeNotebookList", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RecipeNotebookListRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RecipeNotebookListRow { columns: row.columns.clone() }
}
}
pub struct RecipeNotebookListRow {
columns: Vec<ColumnData>,
}
impl RecipeNotebookListRow {
pub fn Recipe(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Count(&self) -> &ColumnData {
&self.columns[1]
}
}
