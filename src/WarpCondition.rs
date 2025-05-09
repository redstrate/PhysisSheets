#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
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
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
WarpConditionRow { columns }
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
