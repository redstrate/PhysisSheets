#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct McGuffin {
exd: EXD,
exh: EXH,
}
impl McGuffin {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("McGuffin").unwrap();let exd = game_data.read_excel_sheet("McGuffin", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> McGuffinRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
McGuffinRow { columns: row.columns.clone() }
}
}
pub struct McGuffinRow {
columns: Vec<ColumnData>,
}
impl McGuffinRow {
pub fn UIData(&self) -> &ColumnData {
&self.columns[0]
}
}
