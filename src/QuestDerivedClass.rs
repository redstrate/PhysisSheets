#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestDerivedClass {
exd: EXD,
exh: EXH,
}
impl QuestDerivedClass {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestDerivedClass").unwrap();let exd = game_data.read_excel_sheet("QuestDerivedClass", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestDerivedClassRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestDerivedClassRow { columns: row.columns.clone() }
}
}
pub struct QuestDerivedClassRow {
columns: Vec<ColumnData>,
}
impl QuestDerivedClassRow {
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[0]
}
}
