#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Warp {
exd: EXD,
exh: EXH,
}
impl Warp {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Warp").unwrap();let exd = game_data.read_excel_sheet("Warp", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WarpRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
WarpRow { columns }
}
}
pub struct WarpRow {
columns: Vec<ColumnData>,
}
impl WarpRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Question(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PopRange(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ConditionSuccessEvent(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ConditionFailEvent(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ConfirmEvent(&self) -> &ColumnData {
&self.columns[5]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[6]
}
pub fn WarpCondition(&self) -> &ColumnData {
&self.columns[7]
}
pub fn WarpLogic(&self) -> &ColumnData {
&self.columns[8]
}
pub fn StartCutscene(&self) -> &ColumnData {
&self.columns[9]
}
pub fn EndCutscene(&self) -> &ColumnData {
&self.columns[10]
}
pub fn CanSkipCutscene(&self) -> &ColumnData {
&self.columns[11]
}
}
