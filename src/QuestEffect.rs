#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestEffect {
exd: EXD,
exh: EXH,
}
impl QuestEffect {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestEffect").unwrap();let exd = game_data.read_excel_sheet("QuestEffect", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestEffectRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestEffectRow { columns: row.columns.clone() }
}
}
pub struct QuestEffectRow {
columns: Vec<ColumnData>,
}
impl QuestEffectRow {
pub fn UnknownStruct(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[3]
}
}
