#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIDisposalShopUICategory {
exd: EXD,
exh: EXH,
}
impl MJIDisposalShopUICategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIDisposalShopUICategory").unwrap();let exd = game_data.read_excel_sheet("MJIDisposalShopUICategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIDisposalShopUICategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIDisposalShopUICategoryRow { columns: row.columns.clone() }
}
}
pub struct MJIDisposalShopUICategoryRow {
columns: Vec<ColumnData>,
}
impl MJIDisposalShopUICategoryRow {
pub fn Category(&self) -> &ColumnData {
&self.columns[0]
}
}
