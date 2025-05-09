#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringItemPoint {
exd: EXD,
exh: EXH,
}
impl GatheringItemPoint {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringItemPoint").unwrap();let exd = game_data.read_excel_sheet("GatheringItemPoint", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringItemPointRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringItemPointRow { columns: row.columns.clone() }
}
}
pub struct GatheringItemPointRow {
columns: Vec<ColumnData>,
}
impl GatheringItemPointRow {
pub fn GatheringPoint(&self) -> &ColumnData {
&self.columns[0]
}
}
