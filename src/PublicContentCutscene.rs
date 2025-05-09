#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PublicContentCutscene {
exd: EXD,
exh: EXH,
}
impl PublicContentCutscene {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PublicContentCutscene").unwrap();let exd = game_data.read_excel_sheet("PublicContentCutscene", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PublicContentCutsceneRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PublicContentCutsceneRow { columns: row.columns.clone() }
}
}
pub struct PublicContentCutsceneRow {
columns: Vec<ColumnData>,
}
impl PublicContentCutsceneRow {
pub fn Cutscene(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Cutscene2(&self) -> &ColumnData {
&self.columns[1]
}
}
