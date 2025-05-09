#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LeveString {
exd: EXD,
exh: EXH,
}
impl LeveString {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LeveString").unwrap();let exd = game_data.read_excel_sheet("LeveString", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LeveStringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LeveStringRow { columns: row.columns.clone() }
}
}
pub struct LeveStringRow {
columns: Vec<ColumnData>,
}
impl LeveStringRow {
pub fn Objective(&self) -> &ColumnData {
&self.columns[0]
}
}
