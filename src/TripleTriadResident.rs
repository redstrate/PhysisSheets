#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadResident {
exd: EXD,
exh: EXH,
}
impl TripleTriadResident {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadResident").unwrap();let exd = game_data.read_excel_sheet("TripleTriadResident", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadResidentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadResidentRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadResidentRow {
columns: Vec<ColumnData>,
}
impl TripleTriadResidentRow {
pub fn Order(&self) -> &ColumnData {
&self.columns[0]
}
}
