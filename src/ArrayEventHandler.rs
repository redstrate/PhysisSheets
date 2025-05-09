#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ArrayEventHandler {
exd: EXD,
exh: EXH,
}
impl ArrayEventHandler {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ArrayEventHandler").unwrap();let exd = game_data.read_excel_sheet("ArrayEventHandler", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ArrayEventHandlerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ArrayEventHandlerRow { columns: row.columns.clone() }
}
}
pub struct ArrayEventHandlerRow {
columns: Vec<ColumnData>,
}
impl ArrayEventHandlerRow {
pub fn Data(&self) -> &ColumnData {
&self.columns[0]
}
}
