#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BGMScene {
exd: EXD,
exh: EXH,
}
impl BGMScene {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BGMScene").unwrap();let exd = game_data.read_excel_sheet("BGMScene", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BGMSceneRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BGMSceneRow { columns: row.columns.clone() }
}
}
pub struct BGMSceneRow {
columns: Vec<ColumnData>,
}
impl BGMSceneRow {
pub fn EnableDisableRestart(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Resume(&self) -> &ColumnData {
&self.columns[1]
}
pub fn EnablePassEnd(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ForceAutoReset(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IgnoreBattle(&self) -> &ColumnData {
&self.columns[4]
}
}
