#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BeastReputationRank {
exd: EXD,
exh: EXH,
}
impl BeastReputationRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BeastReputationRank").unwrap();let exd = game_data.read_excel_sheet("BeastReputationRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BeastReputationRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BeastReputationRankRow { columns: row.columns.clone() }
}
}
pub struct BeastReputationRankRow {
columns: Vec<ColumnData>,
}
impl BeastReputationRankRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AlliedNames(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Color(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredReputation(&self) -> &ColumnData {
&self.columns[3]
}
}
