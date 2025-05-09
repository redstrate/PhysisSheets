#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RecipeLevelTable {
exd: EXD,
exh: EXH,
}
impl RecipeLevelTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("RecipeLevelTable")?;let exd = game_data.read_excel_sheet("RecipeLevelTable", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<RecipeLevelTableRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RecipeLevelTableRow { columns })
}
}
pub struct RecipeLevelTableRow {
columns: Vec<ColumnData>,
}
impl RecipeLevelTableRow {
pub fn Quality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SuggestedCraftsmanship(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Difficulty(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Durability(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ConditionsFlag(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ProgressDivider(&self) -> &ColumnData {
&self.columns[7]
}
pub fn QualityDivider(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ProgressModifier(&self) -> &ColumnData {
&self.columns[9]
}
pub fn QualityModifier(&self) -> &ColumnData {
&self.columns[10]
}
}
