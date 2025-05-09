#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftDraftCategory {
exd: EXD,
exh: EXH,
}
impl CompanyCraftDraftCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftDraftCategory").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftDraftCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftDraftCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftDraftCategoryRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftDraftCategoryRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftDraftCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CompanyCraftType(&self) -> &ColumnData {
&self.columns[1]
}
}
