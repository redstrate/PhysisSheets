#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SubmarineRank {
exd: EXD,
exh: EXH,
}
impl SubmarineRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SubmarineRank").unwrap();let exd = game_data.read_excel_sheet("SubmarineRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SubmarineRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
SubmarineRankRow { columns }
}
}
pub struct SubmarineRankRow {
columns: Vec<ColumnData>,
}
impl SubmarineRankRow {
pub fn ExpToNext(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Capacity(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SurveillanceBonus(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RetrievalBonus(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SpeedBonus(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RangeBonus(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FavorBonus(&self) -> &ColumnData {
&self.columns[6]
}
}
