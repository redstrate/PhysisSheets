#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RacingChocoboParam {
exd: EXD,
exh: EXH,
}
impl RacingChocoboParam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RacingChocoboParam").unwrap();let exd = game_data.read_excel_sheet("RacingChocoboParam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RacingChocoboParamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RacingChocoboParamRow { columns: row.columns.clone() }
}
}
pub struct RacingChocoboParamRow {
columns: Vec<ColumnData>,
}
impl RacingChocoboParamRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
