#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct JournalCategory {
exd: EXD,
exh: EXH,
}
impl JournalCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("JournalCategory").unwrap();let exd = game_data.read_excel_sheet("JournalCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> JournalCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
JournalCategoryRow { columns: row.columns.clone() }
}
}
pub struct JournalCategoryRow {
columns: Vec<ColumnData>,
}
impl JournalCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SeparateType(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DataType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn JournalSection(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MapCondition(&self) -> &ColumnData {
&self.columns[4]
}
}
