#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MYCTemporaryItemUICategory {
exd: EXD,
exh: EXH,
}
impl MYCTemporaryItemUICategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MYCTemporaryItemUICategory").unwrap();let exd = game_data.read_excel_sheet("MYCTemporaryItemUICategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MYCTemporaryItemUICategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MYCTemporaryItemUICategoryRow { columns: row.columns.clone() }
}
}
pub struct MYCTemporaryItemUICategoryRow {
columns: Vec<ColumnData>,
}
impl MYCTemporaryItemUICategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
