#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct YardCatalogItemList {
exd: EXD,
exh: EXH,
}
impl YardCatalogItemList {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("YardCatalogItemList").unwrap();let exd = game_data.read_excel_sheet("YardCatalogItemList", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> YardCatalogItemListRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
YardCatalogItemListRow { columns: row.columns.clone() }
}
}
pub struct YardCatalogItemListRow {
columns: Vec<ColumnData>,
}
impl YardCatalogItemListRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Patch(&self) -> &ColumnData {
&self.columns[2]
}
}
