#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BNpcState {
exd: EXD,
exh: EXH,
}
impl BNpcState {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BNpcState").unwrap();let exd = game_data.read_excel_sheet("BNpcState", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BNpcStateRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
BNpcStateRow { columns }
}
}
pub struct BNpcStateRow {
columns: Vec<ColumnData>,
}
impl BNpcStateRow {
pub fn Scale(&self) -> &ColumnData {
&self.columns[0]
}
pub fn LoopTimeline(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Idle(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Attribute0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Attribute1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Attribute2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn OverRay(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn AttributeFlag0(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AttributeFlag1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn AttributeFlag2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[14]
}
}
