#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TribeSheet {
exd: EXD,
exh: EXH,
}
impl TribeSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Tribe")?;let exd = game_data.read_excel_sheet("Tribe", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<TribeRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TribeRow { columns })
}
}
pub struct TribeRow {
columns: Vec<ColumnData>,
}
impl TribeRow {
pub fn Masculine(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Feminine(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Hp(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Mp(&self) -> &ColumnData {
&self.columns[3]
}
pub fn STR(&self) -> &ColumnData {
&self.columns[4]
}
pub fn VIT(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DEX(&self) -> &ColumnData {
&self.columns[6]
}
pub fn INT(&self) -> &ColumnData {
&self.columns[7]
}
pub fn MND(&self) -> &ColumnData {
&self.columns[8]
}
pub fn PIE(&self) -> &ColumnData {
&self.columns[9]
}
}
