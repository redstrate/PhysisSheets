#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct WKSMissionToDo {
exd: EXD,
exh: EXH,
}
impl WKSMissionToDo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSMissionToDo").unwrap();let exd = game_data.read_excel_sheet("WKSMissionToDo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSMissionToDoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
WKSMissionToDoRow { columns }
}
}
pub struct WKSMissionToDoRow {
columns: Vec<ColumnData>,
}
impl WKSMissionToDoRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredItem(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RequiredItemQuantity(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[15]
}
}
