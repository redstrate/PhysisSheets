#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GFateClimbing2 {
exd: EXD,
exh: EXH,
}
impl GFateClimbing2 {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GFateClimbing2").unwrap();let exd = game_data.read_excel_sheet("GFateClimbing2", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GFateClimbing2Row {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GFateClimbing2Row { columns: row.columns.clone() }
}
}
pub struct GFateClimbing2Row {
columns: Vec<ColumnData>,
}
impl GFateClimbing2Row {
pub fn ContentEntry(&self) -> &ColumnData {
&self.columns[0]
}
}
