#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TreasureHuntRank {
exd: EXD,
exh: EXH,
}
impl TreasureHuntRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TreasureHuntRank").unwrap();let exd = game_data.read_excel_sheet("TreasureHuntRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TreasureHuntRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TreasureHuntRankRow { columns: row.columns.clone() }
}
}
pub struct TreasureHuntRankRow {
columns: Vec<ColumnData>,
}
impl TreasureHuntRankRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemName(&self) -> &ColumnData {
&self.columns[1]
}
pub fn KeyItemName(&self) -> &ColumnData {
&self.columns[2]
}
pub fn InstanceMap(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn MaxPartySize(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TreasureHuntTexture(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[8]
}
}
