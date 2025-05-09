#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJICraftworksPopularityType {
exd: EXD,
exh: EXH,
}
impl MJICraftworksPopularityType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICraftworksPopularityType").unwrap();let exd = game_data.read_excel_sheet("MJICraftworksPopularityType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICraftworksPopularityTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJICraftworksPopularityTypeRow { columns: row.columns.clone() }
}
}
pub struct MJICraftworksPopularityTypeRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksPopularityTypeRow {
pub fn Ratio(&self) -> &ColumnData {
&self.columns[0]
}
}
