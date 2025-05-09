#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Frontline03 {
exd: EXD,
exh: EXH,
}
impl Frontline03 {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Frontline03").unwrap();let exd = game_data.read_excel_sheet("Frontline03", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> Frontline03Row {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
Frontline03Row { columns: row.columns.clone() }
}
}
pub struct Frontline03Row {
columns: Vec<ColumnData>,
}
impl Frontline03Row {
pub fn OvooData(&self) -> &ColumnData {
&self.columns[0]
}
}
