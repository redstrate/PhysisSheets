#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentsTutorial {
exd: EXD,
exh: EXH,
}
impl ContentsTutorial {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentsTutorial").unwrap();let exd = game_data.read_excel_sheet("ContentsTutorial", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentsTutorialRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentsTutorialRow { columns: row.columns.clone() }
}
}
pub struct ContentsTutorialRow {
columns: Vec<ColumnData>,
}
impl ContentsTutorialRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Page(&self) -> &ColumnData {
&self.columns[3]
}
}
