#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TreasureSpot {
exd: EXD,
exh: EXH,
}
impl TreasureSpot {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TreasureSpot").unwrap();let exd = game_data.read_excel_sheet("TreasureSpot", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TreasureSpotRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TreasureSpotRow { columns: row.columns.clone() }
}
}
pub struct TreasureSpotRow {
columns: Vec<ColumnData>,
}
impl TreasureSpotRow {
pub fn MapOffsetX(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MapOffsetY(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[2]
}
}
