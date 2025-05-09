#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AOZContent {
exd: EXD,
exh: EXH,
}
impl AOZContent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZContent").unwrap();let exd = game_data.read_excel_sheet("AOZContent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZContentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
AOZContentRow { columns }
}
}
pub struct AOZContentRow {
columns: Vec<ColumnData>,
}
impl AOZContentRow {
pub fn GilReward(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AlliedSealsReward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TomestonesReward(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ContentEntry(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StandardFinishTime(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IdealFinishTime(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Act1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Act2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Act3(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Act1FightType(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Act2FightType(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Act3FightType(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ArenaType1(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ArenaType2(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ArenaType3(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[18]
}
}
