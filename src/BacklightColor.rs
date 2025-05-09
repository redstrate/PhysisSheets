#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BacklightColor {
exd: EXD,
exh: EXH,
}
impl BacklightColor {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BacklightColor").unwrap();let exd = game_data.read_excel_sheet("BacklightColor", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BacklightColorRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BacklightColorRow { columns: row.columns.clone() }
}
}
pub struct BacklightColorRow {
columns: Vec<ColumnData>,
}
impl BacklightColorRow {
pub fn Color(&self) -> &ColumnData {
&self.columns[0]
}
}
