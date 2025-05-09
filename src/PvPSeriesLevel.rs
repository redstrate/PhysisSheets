#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PvPSeriesLevel {
exd: EXD,
exh: EXH,
}
impl PvPSeriesLevel {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PvPSeriesLevel").unwrap();let exd = game_data.read_excel_sheet("PvPSeriesLevel", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PvPSeriesLevelRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PvPSeriesLevelRow { columns: row.columns.clone() }
}
}
pub struct PvPSeriesLevelRow {
columns: Vec<ColumnData>,
}
impl PvPSeriesLevelRow {
pub fn ExpToNext(&self) -> &ColumnData {
&self.columns[0]
}
}
