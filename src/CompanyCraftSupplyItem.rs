#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftSupplyItem {
exd: EXD,
exh: EXH,
}
impl CompanyCraftSupplyItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftSupplyItem").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftSupplyItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftSupplyItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftSupplyItemRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftSupplyItemRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftSupplyItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}
