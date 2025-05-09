#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSParam {
exd: EXD,
exh: EXH,
}
impl WKSParam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSParam").unwrap();let exd = game_data.read_excel_sheet("WKSParam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSParamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSParamRow { columns: row.columns.clone() }
}
}
pub struct WKSParamRow {
columns: Vec<ColumnData>,
}
impl WKSParamRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
