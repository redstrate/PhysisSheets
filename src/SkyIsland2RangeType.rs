#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SkyIsland2RangeType {
exd: EXD,
exh: EXH,
}
impl SkyIsland2RangeType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SkyIsland2RangeType").unwrap();let exd = game_data.read_excel_sheet("SkyIsland2RangeType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SkyIsland2RangeTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SkyIsland2RangeTypeRow { columns: row.columns.clone() }
}
}
pub struct SkyIsland2RangeTypeRow {
columns: Vec<ColumnData>,
}
impl SkyIsland2RangeTypeRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
}
