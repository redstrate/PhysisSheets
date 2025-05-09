#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCHierarchy {
exd: EXD,
exh: EXH,
}
impl FCHierarchy {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCHierarchy").unwrap();let exd = game_data.read_excel_sheet("FCHierarchy", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCHierarchyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCHierarchyRow { columns: row.columns.clone() }
}
}
pub struct FCHierarchyRow {
columns: Vec<ColumnData>,
}
impl FCHierarchyRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
