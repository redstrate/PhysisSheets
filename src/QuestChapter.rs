#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestChapter {
exd: EXD,
exh: EXH,
}
impl QuestChapter {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestChapter").unwrap();let exd = game_data.read_excel_sheet("QuestChapter", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestChapterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestChapterRow { columns: row.columns.clone() }
}
}
pub struct QuestChapterRow {
columns: Vec<ColumnData>,
}
impl QuestChapterRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Redo(&self) -> &ColumnData {
&self.columns[1]
}
}
