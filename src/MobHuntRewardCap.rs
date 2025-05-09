#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MobHuntRewardCap {
exd: EXD,
exh: EXH,
}
impl MobHuntRewardCap {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MobHuntRewardCap").unwrap();let exd = game_data.read_excel_sheet("MobHuntRewardCap", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MobHuntRewardCapRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MobHuntRewardCapRow { columns: row.columns.clone() }
}
}
pub struct MobHuntRewardCapRow {
columns: Vec<ColumnData>,
}
impl MobHuntRewardCapRow {
pub fn ExpCap(&self) -> &ColumnData {
&self.columns[0]
}
}
