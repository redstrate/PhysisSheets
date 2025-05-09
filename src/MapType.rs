#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MapType {
exd: EXD,
exh: EXH,
}
impl MapType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MapType").unwrap();let exd = game_data.read_excel_sheet("MapType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapTypeRow { columns: row.columns.clone() }
}
}
pub struct MapTypeRow {
columns: Vec<ColumnData>,
}
impl MapTypeRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
