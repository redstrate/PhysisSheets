#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RetainerTaskRandom {
exd: EXD,
exh: EXH,
}
impl RetainerTaskRandom {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RetainerTaskRandom").unwrap();let exd = game_data.read_excel_sheet("RetainerTaskRandom", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RetainerTaskRandomRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RetainerTaskRandomRow { columns: row.columns.clone() }
}
}
pub struct RetainerTaskRandomRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskRandomRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Requirement(&self) -> &ColumnData {
&self.columns[1]
}
}
