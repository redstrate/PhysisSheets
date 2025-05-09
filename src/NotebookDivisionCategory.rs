#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct NotebookDivisionCategory {
exd: EXD,
exh: EXH,
}
impl NotebookDivisionCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("NotebookDivisionCategory").unwrap();let exd = game_data.read_excel_sheet("NotebookDivisionCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> NotebookDivisionCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
NotebookDivisionCategoryRow { columns: row.columns.clone() }
}
}
pub struct NotebookDivisionCategoryRow {
columns: Vec<ColumnData>,
}
impl NotebookDivisionCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Index(&self) -> &ColumnData {
&self.columns[1]
}
}
