#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TraitTransient {
exd: EXD,
exh: EXH,
}
impl TraitTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TraitTransient").unwrap();let exd = game_data.read_excel_sheet("TraitTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TraitTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TraitTransientRow { columns: row.columns.clone() }
}
}
pub struct TraitTransientRow {
columns: Vec<ColumnData>,
}
impl TraitTransientRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
}
