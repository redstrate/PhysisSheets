#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadCardRarity {
exd: EXD,
exh: EXH,
}
impl TripleTriadCardRarity {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadCardRarity").unwrap();let exd = game_data.read_excel_sheet("TripleTriadCardRarity", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadCardRarityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadCardRarityRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadCardRarityRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCardRarityRow {
pub fn Stars(&self) -> &ColumnData {
&self.columns[0]
}
}
