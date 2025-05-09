#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemSortCategory {
exd: EXD,
exh: EXH,
}
impl ItemSortCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemSortCategory").unwrap();let exd = game_data.read_excel_sheet("ItemSortCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemSortCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemSortCategoryRow { columns: row.columns.clone() }
}
}
pub struct ItemSortCategoryRow {
columns: Vec<ColumnData>,
}
impl ItemSortCategoryRow {
pub fn Param(&self) -> &ColumnData {
&self.columns[0]
}
}
