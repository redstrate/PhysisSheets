#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DescriptionString {
exd: EXD,
exh: EXH,
}
impl DescriptionString {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DescriptionString").unwrap();let exd = game_data.read_excel_sheet("DescriptionString", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DescriptionStringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DescriptionStringRow { columns: row.columns.clone() }
}
}
pub struct DescriptionStringRow {
columns: Vec<ColumnData>,
}
impl DescriptionStringRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
