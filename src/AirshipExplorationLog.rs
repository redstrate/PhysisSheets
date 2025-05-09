#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AirshipExplorationLog {
exd: EXD,
exh: EXH,
}
impl AirshipExplorationLog {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AirshipExplorationLog").unwrap();let exd = game_data.read_excel_sheet("AirshipExplorationLog", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AirshipExplorationLogRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AirshipExplorationLogRow { columns: row.columns.clone() }
}
}
pub struct AirshipExplorationLogRow {
columns: Vec<ColumnData>,
}
impl AirshipExplorationLogRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
