#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestBattle {
exd: EXD,
exh: EXH,
}
impl QuestBattle {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestBattle").unwrap();let exd = game_data.read_excel_sheet("QuestBattle", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestBattleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestBattleRow { columns: row.columns.clone() }
}
}
pub struct QuestBattleRow {
columns: Vec<ColumnData>,
}
impl QuestBattleRow {
pub fn QuestBattleParams(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LevelSync(&self) -> &ColumnData {
&self.columns[3]
}
pub fn QuestBattleScene(&self) -> &ColumnData {
&self.columns[4]
}
}
