#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanionMove {
exd: EXD,
exh: EXH,
}
impl CompanionMove {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanionMove").unwrap();let exd = game_data.read_excel_sheet("CompanionMove", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanionMoveRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanionMoveRow { columns: row.columns.clone() }
}
}
pub struct CompanionMoveRow {
columns: Vec<ColumnData>,
}
impl CompanionMoveRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
