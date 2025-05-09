#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Achievement {
exd: EXD,
exh: EXH,
}
impl Achievement {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Achievement").unwrap();let exd = game_data.read_excel_sheet("Achievement", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AchievementRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
AchievementRow { columns }
}
}
pub struct AchievementRow {
columns: Vec<ColumnData>,
}
impl AchievementRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Data(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Title(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[7]
}
pub fn AchievementCategory(&self) -> &ColumnData {
&self.columns[8]
}
pub fn AchievementTarget(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Points(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[17]
}
pub fn AchievementHideCondition(&self) -> &ColumnData {
&self.columns[18]
}
}
