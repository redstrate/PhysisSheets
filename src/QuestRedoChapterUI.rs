#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestRedoChapterUI {
exd: EXD,
exh: EXH,
}
impl QuestRedoChapterUI {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestRedoChapterUI").unwrap();let exd = game_data.read_excel_sheet("QuestRedoChapterUI", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestRedoChapterUIRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestRedoChapterUIRow { columns: row.columns.clone() }
}
}
pub struct QuestRedoChapterUIRow {
columns: Vec<ColumnData>,
}
impl QuestRedoChapterUIRow {
pub fn ChapterName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ChapterPart(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Transient(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn QuestRedoUISmall(&self) -> &ColumnData {
&self.columns[5]
}
pub fn QuestRedoUILarge(&self) -> &ColumnData {
&self.columns[6]
}
pub fn QuestRedoUIWide(&self) -> &ColumnData {
&self.columns[7]
}
pub fn UITab(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
}
