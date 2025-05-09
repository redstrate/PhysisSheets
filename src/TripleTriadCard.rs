#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadCard {
exd: EXD,
exh: EXH,
}
impl TripleTriadCard {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadCard").unwrap();let exd = game_data.read_excel_sheet("TripleTriadCard", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadCardRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadCardRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadCardRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCardRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[8]
}
}
