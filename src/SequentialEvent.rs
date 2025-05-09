#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SequentialEvent {
exd: EXD,
exh: EXH,
}
impl SequentialEvent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SequentialEvent").unwrap();let exd = game_data.read_excel_sheet("SequentialEvent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SequentialEventRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SequentialEventRow { columns: row.columns.clone() }
}
}
pub struct SequentialEventRow {
columns: Vec<ColumnData>,
}
impl SequentialEventRow {
pub fn UnknownStruct(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown320(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown321(&self) -> &ColumnData {
&self.columns[3]
}
}
