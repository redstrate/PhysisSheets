#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MultipleHelpString {
exd: EXD,
exh: EXH,
}
impl MultipleHelpString {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MultipleHelpString").unwrap();let exd = game_data.read_excel_sheet("MultipleHelpString", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MultipleHelpStringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MultipleHelpStringRow { columns: row.columns.clone() }
}
}
pub struct MultipleHelpStringRow {
columns: Vec<ColumnData>,
}
impl MultipleHelpStringRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}
