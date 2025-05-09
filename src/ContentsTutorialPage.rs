#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentsTutorialPage {
exd: EXD,
exh: EXH,
}
impl ContentsTutorialPage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentsTutorialPage").unwrap();let exd = game_data.read_excel_sheet("ContentsTutorialPage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentsTutorialPageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentsTutorialPageRow { columns: row.columns.clone() }
}
}
pub struct ContentsTutorialPageRow {
columns: Vec<ColumnData>,
}
impl ContentsTutorialPageRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[1]
}
}
