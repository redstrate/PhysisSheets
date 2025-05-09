#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct World {
exd: EXD,
exh: EXH,
}
impl World {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("World")?;let exd = game_data.read_excel_sheet("World", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<WorldRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WorldRow { columns })
}
}
pub struct WorldRow {
columns: Vec<ColumnData>,
}
impl WorldRow {
pub fn InternalName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Region(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UserType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DataCenter(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IsPublic(&self) -> &ColumnData {
&self.columns[5]
}
}
