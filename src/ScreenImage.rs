#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ScreenImage {
exd: EXD,
exh: EXH,
}
impl ScreenImage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ScreenImage").unwrap();let exd = game_data.read_excel_sheet("ScreenImage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ScreenImageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ScreenImageRow { columns: row.columns.clone() }
}
}
pub struct ScreenImageRow {
columns: Vec<ColumnData>,
}
impl ScreenImageRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Jingle(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Lang(&self) -> &ColumnData {
&self.columns[3]
}
}
