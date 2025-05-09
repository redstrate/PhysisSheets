#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Picture {
exd: EXD,
exh: EXH,
}
impl Picture {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Picture").unwrap();let exd = game_data.read_excel_sheet("Picture", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PictureRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PictureRow { columns: row.columns.clone() }
}
}
pub struct PictureRow {
columns: Vec<ColumnData>,
}
impl PictureRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Signature(&self) -> &ColumnData {
&self.columns[1]
}
}
