#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TutorialHealer {
exd: EXD,
exh: EXH,
}
impl TutorialHealer {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TutorialHealer").unwrap();let exd = game_data.read_excel_sheet("TutorialHealer", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TutorialHealerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TutorialHealerRow { columns: row.columns.clone() }
}
}
pub struct TutorialHealerRow {
columns: Vec<ColumnData>,
}
impl TutorialHealerRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[1]
}
}
