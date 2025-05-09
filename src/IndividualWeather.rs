#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct IndividualWeather {
exd: EXD,
exh: EXH,
}
impl IndividualWeather {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("IndividualWeather").unwrap();let exd = game_data.read_excel_sheet("IndividualWeather", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> IndividualWeatherRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
IndividualWeatherRow { columns: row.columns.clone() }
}
}
pub struct IndividualWeatherRow {
columns: Vec<ColumnData>,
}
impl IndividualWeatherRow {
pub fn IndividualWeatherData(&self) -> &ColumnData {
&self.columns[0]
}
}
