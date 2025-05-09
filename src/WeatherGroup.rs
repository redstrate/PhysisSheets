#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeatherGroup {
exd: EXD,
exh: EXH,
}
impl WeatherGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeatherGroup").unwrap();let exd = game_data.read_excel_sheet("WeatherGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeatherGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeatherGroupRow { columns: row.columns.clone() }
}
}
pub struct WeatherGroupRow {
columns: Vec<ColumnData>,
}
impl WeatherGroupRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn WeatherRate(&self) -> &ColumnData {
&self.columns[1]
}
}
