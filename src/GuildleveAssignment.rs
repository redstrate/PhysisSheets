#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GuildleveAssignment {
exd: EXD,
exh: EXH,
}
impl GuildleveAssignment {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GuildleveAssignment").unwrap();let exd = game_data.read_excel_sheet("GuildleveAssignment", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GuildleveAssignmentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
GuildleveAssignmentRow { columns }
}
}
pub struct GuildleveAssignmentRow {
columns: Vec<ColumnData>,
}
impl GuildleveAssignmentRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AssignmentTalk(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[9]
}
}
