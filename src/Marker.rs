#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Marker {
exd: EXD,
exh: EXH,
}
impl Marker {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Marker").unwrap();let exd = game_data.read_excel_sheet("Marker", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MarkerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MarkerRow { columns: row.columns.clone() }
}
}
pub struct MarkerRow {
columns: Vec<ColumnData>,
}
impl MarkerRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SortOrder(&self) -> &ColumnData {
&self.columns[2]
}
}
