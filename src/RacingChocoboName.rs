#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RacingChocoboName {
exd: EXD,
exh: EXH,
}
impl RacingChocoboName {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RacingChocoboName").unwrap();let exd = game_data.read_excel_sheet("RacingChocoboName", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RacingChocoboNameRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RacingChocoboNameRow { columns: row.columns.clone() }
}
}
pub struct RacingChocoboNameRow {
columns: Vec<ColumnData>,
}
impl RacingChocoboNameRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
