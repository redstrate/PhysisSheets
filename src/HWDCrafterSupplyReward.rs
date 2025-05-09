#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDCrafterSupplyReward {
exd: EXD,
exh: EXH,
}
impl HWDCrafterSupplyReward {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDCrafterSupplyReward").unwrap();let exd = game_data.read_excel_sheet("HWDCrafterSupplyReward", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDCrafterSupplyRewardRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDCrafterSupplyRewardRow { columns: row.columns.clone() }
}
}
pub struct HWDCrafterSupplyRewardRow {
columns: Vec<ColumnData>,
}
impl HWDCrafterSupplyRewardRow {
pub fn ExpReward(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ScriptRewardAmount(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Points(&self) -> &ColumnData {
&self.columns[2]
}
}
