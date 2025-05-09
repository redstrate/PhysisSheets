#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MateriaJoinRate {
exd: EXD,
exh: EXH,
}
impl MateriaJoinRate {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MateriaJoinRate").unwrap();let exd = game_data.read_excel_sheet("MateriaJoinRate", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MateriaJoinRateRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MateriaJoinRateRow { columns: row.columns.clone() }
}
}
pub struct MateriaJoinRateRow {
columns: Vec<ColumnData>,
}
impl MateriaJoinRateRow {
pub fn NQOvermeldPercentSlot(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HQOvermeldPercentSlot(&self) -> &ColumnData {
&self.columns[1]
}
}
