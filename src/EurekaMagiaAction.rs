#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaMagiaAction {
exd: EXD,
exh: EXH,
}
impl EurekaMagiaAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaMagiaAction").unwrap();let exd = game_data.read_excel_sheet("EurekaMagiaAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaMagiaActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaMagiaActionRow { columns: row.columns.clone() }
}
}
pub struct EurekaMagiaActionRow {
columns: Vec<ColumnData>,
}
impl EurekaMagiaActionRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MaxUses(&self) -> &ColumnData {
&self.columns[1]
}
}
