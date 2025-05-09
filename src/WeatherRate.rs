#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeatherRate {
exd: EXD,
exh: EXH,
}
impl WeatherRate {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeatherRate").unwrap();let exd = game_data.read_excel_sheet("WeatherRate", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeatherRateRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeatherRateRow { columns: row.columns.clone() }
}
}
pub struct WeatherRateRow {
columns: Vec<ColumnData>,
}
impl WeatherRateRow {
pub fn Weather(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Rate(&self) -> &ColumnData {
&self.columns[1]
}
}
