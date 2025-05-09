#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MobHuntReward {
exd: EXD,
exh: EXH,
}
impl MobHuntReward {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MobHuntReward").unwrap();let exd = game_data.read_excel_sheet("MobHuntReward", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MobHuntRewardRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MobHuntRewardRow { columns: row.columns.clone() }
}
}
pub struct MobHuntRewardRow {
columns: Vec<ColumnData>,
}
impl MobHuntRewardRow {
pub fn ExpReward(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GilReward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CurrencyReward(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Expansion(&self) -> &ColumnData {
&self.columns[3]
}
}
