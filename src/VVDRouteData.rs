#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct VVDRouteData {
exd: EXD,
exh: EXH,
}
impl VVDRouteData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("VVDRouteData").unwrap();let exd = game_data.read_excel_sheet("VVDRouteData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> VVDRouteDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
VVDRouteDataRow { columns: row.columns.clone() }
}
}
pub struct VVDRouteDataRow {
columns: Vec<ColumnData>,
}
impl VVDRouteDataRow {
pub fn NotebookEntry(&self) -> &ColumnData {
&self.columns[0]
}
}
