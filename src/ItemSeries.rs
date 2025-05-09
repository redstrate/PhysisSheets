#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemSeries {
exd: EXD,
exh: EXH,
}
impl ItemSeries {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemSeries").unwrap();let exd = game_data.read_excel_sheet("ItemSeries", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemSeriesRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemSeriesRow { columns: row.columns.clone() }
}
}
pub struct ItemSeriesRow {
columns: Vec<ColumnData>,
}
impl ItemSeriesRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
