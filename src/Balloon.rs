#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Balloon {
exd: EXD,
exh: EXH,
}
impl Balloon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Balloon").unwrap();let exd = game_data.read_excel_sheet("Balloon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BalloonRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BalloonRow { columns: row.columns.clone() }
}
}
pub struct BalloonRow {
columns: Vec<ColumnData>,
}
impl BalloonRow {
pub fn Dialogue(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Slowly(&self) -> &ColumnData {
&self.columns[1]
}
}
