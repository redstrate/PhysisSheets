#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FateEvent {
exd: EXD,
exh: EXH,
}
impl FateEvent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FateEvent").unwrap();let exd = game_data.read_excel_sheet("FateEvent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateEventRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FateEventRow { columns: row.columns.clone() }
}
}
pub struct FateEventRow {
columns: Vec<ColumnData>,
}
impl FateEventRow {
pub fn EventParameters(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Text(&self) -> &ColumnData {
&self.columns[1]
}
}
