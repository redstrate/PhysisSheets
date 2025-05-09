#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TutorialGimmick {
exd: EXD,
exh: EXH,
}
impl TutorialGimmick {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TutorialGimmick").unwrap();let exd = game_data.read_excel_sheet("TutorialGimmick", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TutorialGimmickRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TutorialGimmickRow { columns: row.columns.clone() }
}
}
pub struct TutorialGimmickRow {
columns: Vec<ColumnData>,
}
impl TutorialGimmickRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[1]
}
}
