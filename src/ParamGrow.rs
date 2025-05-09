#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ParamGrow {
exd: EXD,
exh: EXH,
}
impl ParamGrow {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ParamGrow").unwrap();let exd = game_data.read_excel_sheet("ParamGrow", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ParamGrowRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
ParamGrowRow { columns }
}
}
pub struct ParamGrowRow {
columns: Vec<ColumnData>,
}
impl ParamGrowRow {
pub fn ExpToNext(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MpModifier(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BaseSpeed(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LevelModifier(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HuntingLogExpReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn MonsterNoteSeals(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ScaledQuestXP(&self) -> &ColumnData {
&self.columns[6]
}
pub fn HpModifier(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ItemLevelSync(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ProperDungeon(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ProperGuildOrder(&self) -> &ColumnData {
&self.columns[10]
}
pub fn CraftingLevel(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AdditionalActions(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ApplyAction(&self) -> &ColumnData {
&self.columns[13]
}
pub fn QuestExpModifier(&self) -> &ColumnData {
&self.columns[14]
}
}
