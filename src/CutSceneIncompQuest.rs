#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CutSceneIncompQuest {
exd: EXD,
exh: EXH,
}
impl CutSceneIncompQuest {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CutSceneIncompQuest").unwrap();let exd = game_data.read_excel_sheet("CutSceneIncompQuest", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CutSceneIncompQuestRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CutSceneIncompQuestRow { columns: row.columns.clone() }
}
}
pub struct CutSceneIncompQuestRow {
columns: Vec<ColumnData>,
}
impl CutSceneIncompQuestRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
}
