#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct InclusionShopSeries {
exd: EXD,
exh: EXH,
}
impl InclusionShopSeries {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("InclusionShopSeries").unwrap();let exd = game_data.read_excel_sheet("InclusionShopSeries", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> InclusionShopSeriesRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
InclusionShopSeriesRow { columns: row.columns.clone() }
}
}
pub struct InclusionShopSeriesRow {
columns: Vec<ColumnData>,
}
impl InclusionShopSeriesRow {
pub fn SpecialShop(&self) -> &ColumnData {
&self.columns[0]
}
}
