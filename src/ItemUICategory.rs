#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemUICategory {
exd: EXD,
exh: EXH,
}
impl ItemUICategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemUICategory").unwrap();let exd = game_data.read_excel_sheet("ItemUICategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemUICategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemUICategoryRow { columns: row.columns.clone() }
}
}
pub struct ItemUICategoryRow {
columns: Vec<ColumnData>,
}
impl ItemUICategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn OrderMinor(&self) -> &ColumnData {
&self.columns[2]
}
pub fn OrderMajor(&self) -> &ColumnData {
&self.columns[3]
}
}
