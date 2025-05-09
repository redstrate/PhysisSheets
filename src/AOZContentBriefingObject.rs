#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AOZContentBriefingObject {
exd: EXD,
exh: EXH,
}
impl AOZContentBriefingObject {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZContentBriefingObject").unwrap();let exd = game_data.read_excel_sheet("AOZContentBriefingObject", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZContentBriefingObjectRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
AOZContentBriefingObjectRow { columns }
}
}
pub struct AOZContentBriefingObjectRow {
columns: Vec<ColumnData>,
}
impl AOZContentBriefingObjectRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
