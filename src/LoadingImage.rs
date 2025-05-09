#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LoadingImage {
exd: EXD,
exh: EXH,
}
impl LoadingImage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LoadingImage").unwrap();let exd = game_data.read_excel_sheet("LoadingImage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LoadingImageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LoadingImageRow { columns: row.columns.clone() }
}
}
pub struct LoadingImageRow {
columns: Vec<ColumnData>,
}
impl LoadingImageRow {
pub fn FileName(&self) -> &ColumnData {
&self.columns[0]
}
}
