#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BGMFade {
exd: EXD,
exh: EXH,
}
impl BGMFade {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BGMFade")?;let exd = game_data.read_excel_sheet("BGMFade", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<BGMFadeRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BGMFadeRow { columns })
}
}
pub struct BGMFadeRow {
columns: Vec<ColumnData>,
}
impl BGMFadeRow {
pub fn SceneOut(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SceneIn(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BGMFadeType(&self) -> &ColumnData {
&self.columns[2]
}
}
