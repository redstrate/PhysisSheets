#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJICraftworksRankRatio {
exd: EXD,
exh: EXH,
}
impl MJICraftworksRankRatio {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICraftworksRankRatio").unwrap();let exd = game_data.read_excel_sheet("MJICraftworksRankRatio", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICraftworksRankRatioRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJICraftworksRankRatioRow { columns: row.columns.clone() }
}
}
pub struct MJICraftworksRankRatioRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksRankRatioRow {
pub fn Ratio(&self) -> &ColumnData {
&self.columns[0]
}
}
