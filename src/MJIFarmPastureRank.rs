#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIFarmPastureRank {
exd: EXD,
exh: EXH,
}
impl MJIFarmPastureRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIFarmPastureRank").unwrap();let exd = game_data.read_excel_sheet("MJIFarmPastureRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIFarmPastureRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIFarmPastureRankRow { columns: row.columns.clone() }
}
}
pub struct MJIFarmPastureRankRow {
columns: Vec<ColumnData>,
}
impl MJIFarmPastureRankRow {
pub fn RankData(&self) -> &ColumnData {
&self.columns[0]
}
}
