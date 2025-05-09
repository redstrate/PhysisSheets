#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestRedoChapterUITab {
exd: EXD,
exh: EXH,
}
impl QuestRedoChapterUITab {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestRedoChapterUITab").unwrap();let exd = game_data.read_excel_sheet("QuestRedoChapterUITab", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestRedoChapterUITabRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestRedoChapterUITabRow { columns: row.columns.clone() }
}
}
pub struct QuestRedoChapterUITabRow {
columns: Vec<ColumnData>,
}
impl QuestRedoChapterUITabRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
