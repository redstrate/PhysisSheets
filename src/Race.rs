#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Race {
exd: EXD,
exh: EXH,
}
impl Race {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Race").unwrap();let exd = game_data.read_excel_sheet("Race", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
RaceRow { columns }
}
}
pub struct RaceRow {
columns: Vec<ColumnData>,
}
impl RaceRow {
pub fn Masculine(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Feminine(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RSEMBody(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RSEFBody(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RSEMHands(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RSEFHands(&self) -> &ColumnData {
&self.columns[5]
}
pub fn RSEMLegs(&self) -> &ColumnData {
&self.columns[6]
}
pub fn RSEFLegs(&self) -> &ColumnData {
&self.columns[7]
}
pub fn RSEMFeet(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RSEFFeet(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ExPac(&self) -> &ColumnData {
&self.columns[11]
}
}
