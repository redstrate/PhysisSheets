#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MovieSubtitleVoyage {
exd: EXD,
exh: EXH,
}
impl MovieSubtitleVoyage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MovieSubtitleVoyage").unwrap();let exd = game_data.read_excel_sheet("MovieSubtitleVoyage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MovieSubtitleVoyageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MovieSubtitleVoyageRow { columns: row.columns.clone() }
}
}
pub struct MovieSubtitleVoyageRow {
columns: Vec<ColumnData>,
}
impl MovieSubtitleVoyageRow {
pub fn StartTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EndTime(&self) -> &ColumnData {
&self.columns[1]
}
}
