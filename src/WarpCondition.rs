#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WarpCondition {
exd: EXD,
exh: EXH,
}
impl WarpCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WarpCondition").unwrap();let exd = game_data.read_excel_sheet("WarpCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WarpConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WarpConditionRow { columns: row.columns.clone() }
}
}
pub struct WarpConditionRow {
columns: Vec<ColumnData>,
}
impl WarpConditionRow {
pub fn RequiredQuest1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredQuest2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredQuest3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredQuest4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Gil(&self) -> &ColumnData {
&self.columns[4]
}
pub fn QuestReward(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ClassLevel(&self) -> &ColumnData {
&self.columns[6]
}
pub fn CompleteParam(&self) -> &ColumnData {
&self.columns[7]
}
}
