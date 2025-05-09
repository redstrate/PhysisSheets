#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadCardObtain {
exd: EXD,
exh: EXH,
}
impl TripleTriadCardObtain {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadCardObtain").unwrap();let exd = game_data.read_excel_sheet("TripleTriadCardObtain", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadCardObtainRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadCardObtainRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadCardObtainRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCardObtainRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Text(&self) -> &ColumnData {
&self.columns[1]
}
}
