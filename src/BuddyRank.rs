#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BuddyRank {
exd: EXD,
exh: EXH,
}
impl BuddyRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BuddyRank").unwrap();let exd = game_data.read_excel_sheet("BuddyRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BuddyRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BuddyRankRow { columns: row.columns.clone() }
}
}
pub struct BuddyRankRow {
columns: Vec<ColumnData>,
}
impl BuddyRankRow {
pub fn ExpRequired(&self) -> &ColumnData {
&self.columns[0]
}
}
