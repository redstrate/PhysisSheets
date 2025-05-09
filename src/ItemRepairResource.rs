#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemRepairResource {
exd: EXD,
exh: EXH,
}
impl ItemRepairResource {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemRepairResource").unwrap();let exd = game_data.read_excel_sheet("ItemRepairResource", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemRepairResourceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemRepairResourceRow { columns: row.columns.clone() }
}
}
pub struct ItemRepairResourceRow {
columns: Vec<ColumnData>,
}
impl ItemRepairResourceRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}
