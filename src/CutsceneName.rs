#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CutsceneName {
exd: EXD,
exh: EXH,
}
impl CutsceneName {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CutsceneName").unwrap();let exd = game_data.read_excel_sheet("CutsceneName", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CutsceneNameRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CutsceneNameRow { columns: row.columns.clone() }
}
}
pub struct CutsceneNameRow {
columns: Vec<ColumnData>,
}
impl CutsceneNameRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
