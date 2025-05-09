#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentEventItem {
exd: EXD,
exh: EXH,
}
impl ContentEventItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentEventItem").unwrap();let exd = game_data.read_excel_sheet("ContentEventItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentEventItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentEventItemRow { columns: row.columns.clone() }
}
}
pub struct ContentEventItemRow {
columns: Vec<ColumnData>,
}
impl ContentEventItemRow {
pub fn EventItem(&self) -> &ColumnData {
&self.columns[0]
}
}
