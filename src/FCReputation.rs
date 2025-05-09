#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCReputation {
exd: EXD,
exh: EXH,
}
impl FCReputation {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCReputation").unwrap();let exd = game_data.read_excel_sheet("FCReputation", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCReputationRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCReputationRow { columns: row.columns.clone() }
}
}
pub struct FCReputationRow {
columns: Vec<ColumnData>,
}
impl FCReputationRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PointsToNext(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredPoints(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Color(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DiscountRate(&self) -> &ColumnData {
&self.columns[4]
}
}
