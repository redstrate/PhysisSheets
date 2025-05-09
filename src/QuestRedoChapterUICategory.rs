#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestRedoChapterUICategory {
exd: EXD,
exh: EXH,
}
impl QuestRedoChapterUICategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestRedoChapterUICategory").unwrap();let exd = game_data.read_excel_sheet("QuestRedoChapterUICategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestRedoChapterUICategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestRedoChapterUICategoryRow { columns: row.columns.clone() }
}
}
pub struct QuestRedoChapterUICategoryRow {
columns: Vec<ColumnData>,
}
impl QuestRedoChapterUICategoryRow {
pub fn Expac(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
