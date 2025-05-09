#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BannerCondition {
exd: EXD,
exh: EXH,
}
impl BannerCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BannerCondition").unwrap();let exd = game_data.read_excel_sheet("BannerCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BannerConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
BannerConditionRow { columns }
}
}
pub struct BannerConditionRow {
columns: Vec<ColumnData>,
}
impl BannerConditionRow {
pub fn UnlockCriteria1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockCriteria2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn UnlockCriteria3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UnlockCriteria4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Prerequisite(&self) -> &ColumnData {
&self.columns[5]
}
pub fn UnlockType1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn UnlockType2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PrerequisiteType(&self) -> &ColumnData {
&self.columns[8]
}
pub fn UnlockHint(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[10]
}
}
