#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LegacyQuest {
exd: EXD,
exh: EXH,
}
impl LegacyQuest {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LegacyQuest").unwrap();let exd = game_data.read_excel_sheet("LegacyQuest", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LegacyQuestRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LegacyQuestRow { columns: row.columns.clone() }
}
}
pub struct LegacyQuestRow {
columns: Vec<ColumnData>,
}
impl LegacyQuestRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn String(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Genre(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LegacyQuestID(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[4]
}
}
