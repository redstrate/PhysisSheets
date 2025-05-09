#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EmoteMode {
exd: EXD,
exh: EXH,
}
impl EmoteMode {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EmoteMode").unwrap();let exd = game_data.read_excel_sheet("EmoteMode", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EmoteModeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EmoteModeRow { columns: row.columns.clone() }
}
}
pub struct EmoteModeRow {
columns: Vec<ColumnData>,
}
impl EmoteModeRow {
pub fn StartEmote(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EndEmote(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ConditionMode(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Move(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Camera(&self) -> &ColumnData {
&self.columns[4]
}
pub fn EndOnRotate(&self) -> &ColumnData {
&self.columns[5]
}
pub fn EndOnEmote(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
}
