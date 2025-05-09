#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJICraftworksPopularity {
exd: EXD,
exh: EXH,
}
impl MJICraftworksPopularity {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICraftworksPopularity").unwrap();let exd = game_data.read_excel_sheet("MJICraftworksPopularity", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICraftworksPopularityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJICraftworksPopularityRow { columns: row.columns.clone() }
}
}
pub struct MJICraftworksPopularityRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksPopularityRow {
pub fn Popularity(&self) -> &ColumnData {
&self.columns[0]
}
}
