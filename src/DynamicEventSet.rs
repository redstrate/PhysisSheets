#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DynamicEventSet {
exd: EXD,
exh: EXH,
}
impl DynamicEventSet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DynamicEventSet").unwrap();let exd = game_data.read_excel_sheet("DynamicEventSet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DynamicEventSetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DynamicEventSetRow { columns: row.columns.clone() }
}
}
pub struct DynamicEventSetRow {
columns: Vec<ColumnData>,
}
impl DynamicEventSetRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
