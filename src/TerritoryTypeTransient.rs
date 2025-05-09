#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TerritoryTypeTransient {
exd: EXD,
exh: EXH,
}
impl TerritoryTypeTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TerritoryTypeTransient").unwrap();let exd = game_data.read_excel_sheet("TerritoryTypeTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TerritoryTypeTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TerritoryTypeTransientRow { columns: row.columns.clone() }
}
}
pub struct TerritoryTypeTransientRow {
columns: Vec<ColumnData>,
}
impl TerritoryTypeTransientRow {
pub fn OffsetZ(&self) -> &ColumnData {
&self.columns[0]
}
}
