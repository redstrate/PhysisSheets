#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeatherReportReplace {
exd: EXD,
exh: EXH,
}
impl WeatherReportReplace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeatherReportReplace").unwrap();let exd = game_data.read_excel_sheet("WeatherReportReplace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeatherReportReplaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeatherReportReplaceRow { columns: row.columns.clone() }
}
}
pub struct WeatherReportReplaceRow {
columns: Vec<ColumnData>,
}
impl WeatherReportReplaceRow {
pub fn PlaceNameSub(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PlaceNameParent(&self) -> &ColumnData {
&self.columns[1]
}
}
