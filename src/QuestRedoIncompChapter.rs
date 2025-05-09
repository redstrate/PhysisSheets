#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestRedoIncompChapter {
exd: EXD,
exh: EXH,
}
impl QuestRedoIncompChapter {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestRedoIncompChapter").unwrap();let exd = game_data.read_excel_sheet("QuestRedoIncompChapter", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestRedoIncompChapterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestRedoIncompChapterRow { columns: row.columns.clone() }
}
}
pub struct QuestRedoIncompChapterRow {
columns: Vec<ColumnData>,
}
impl QuestRedoIncompChapterRow {
pub fn Chapter(&self) -> &ColumnData {
&self.columns[0]
}
}
