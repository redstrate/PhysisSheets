#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadCompetition {
exd: EXD,
exh: EXH,
}
impl TripleTriadCompetition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadCompetition").unwrap();let exd = game_data.read_excel_sheet("TripleTriadCompetition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadCompetitionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadCompetitionRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadCompetitionRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCompetitionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
