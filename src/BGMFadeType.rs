#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BGMFadeType {
exd: EXD,
exh: EXH,
}
impl BGMFadeType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BGMFadeType").unwrap();let exd = game_data.read_excel_sheet("BGMFadeType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BGMFadeTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
BGMFadeTypeRow { columns }
}
}
pub struct BGMFadeTypeRow {
columns: Vec<ColumnData>,
}
impl BGMFadeTypeRow {
pub fn FadeOutTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn FadeInTime(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FadeInStartTime(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ResumeFadeInTime(&self) -> &ColumnData {
&self.columns[3]
}
}
