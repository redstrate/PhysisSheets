#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MateriaJoinRateGatherCraft {
exd: EXD,
exh: EXH,
}
impl MateriaJoinRateGatherCraft {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MateriaJoinRateGatherCraft").unwrap();let exd = game_data.read_excel_sheet("MateriaJoinRateGatherCraft", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MateriaJoinRateGatherCraftRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MateriaJoinRateGatherCraftRow { columns: row.columns.clone() }
}
}
pub struct MateriaJoinRateGatherCraftRow {
columns: Vec<ColumnData>,
}
impl MateriaJoinRateGatherCraftRow {
pub fn NQOvermeldPercentSlot(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HQOvermeldPercentSlot(&self) -> &ColumnData {
&self.columns[1]
}
}
