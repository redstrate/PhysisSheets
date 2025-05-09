#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestEffectDefine {
exd: EXD,
exh: EXH,
}
impl QuestEffectDefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestEffectDefine").unwrap();let exd = game_data.read_excel_sheet("QuestEffectDefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestEffectDefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestEffectDefineRow { columns: row.columns.clone() }
}
}
pub struct QuestEffectDefineRow {
columns: Vec<ColumnData>,
}
impl QuestEffectDefineRow {
pub fn Effect(&self) -> &ColumnData {
&self.columns[0]
}
}
