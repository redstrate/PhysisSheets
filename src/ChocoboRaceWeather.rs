#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboRaceWeather {
exd: EXD,
exh: EXH,
}
impl ChocoboRaceWeather {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboRaceWeather").unwrap();let exd = game_data.read_excel_sheet("ChocoboRaceWeather", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboRaceWeatherRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboRaceWeatherRow { columns: row.columns.clone() }
}
}
pub struct ChocoboRaceWeatherRow {
columns: Vec<ColumnData>,
}
impl ChocoboRaceWeatherRow {
pub fn WeatherType1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn WeatherType2(&self) -> &ColumnData {
&self.columns[1]
}
}
