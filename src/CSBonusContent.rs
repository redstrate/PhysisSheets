#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CSBonusContent {
exd: EXD,
exh: EXH,
}
impl CSBonusContent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusContent").unwrap();let exd = game_data.read_excel_sheet("CSBonusContent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusContentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
CSBonusContentRow { columns }
}
}
pub struct CSBonusContentRow {
columns: Vec<ColumnData>,
}
impl CSBonusContentRow {
pub fn Score1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Score2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Score3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Score4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Score5(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Content0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Content1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Score0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ContentType(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RewardCount0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RewardCount1(&self) -> &ColumnData {
&self.columns[10]
}
pub fn RewardCount2(&self) -> &ColumnData {
&self.columns[11]
}
pub fn RewardCount3(&self) -> &ColumnData {
&self.columns[12]
}
pub fn RewardCount4(&self) -> &ColumnData {
&self.columns[13]
}
}
