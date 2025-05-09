#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCAuthorityCategory {
exd: EXD,
exh: EXH,
}
impl FCAuthorityCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCAuthorityCategory").unwrap();let exd = game_data.read_excel_sheet("FCAuthorityCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCAuthorityCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCAuthorityCategoryRow { columns: row.columns.clone() }
}
}
pub struct FCAuthorityCategoryRow {
columns: Vec<ColumnData>,
}
impl FCAuthorityCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
