#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct FCActivity {
exd: EXD,
exh: EXH,
}
impl FCActivity {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCActivity").unwrap();let exd = game_data.read_excel_sheet("FCActivity", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCActivityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
FCActivityRow { columns }
}
}
pub struct FCActivityRow {
columns: Vec<ColumnData>,
}
impl FCActivityRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SelfKind(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TargetKind(&self) -> &ColumnData {
&self.columns[2]
}
pub fn NumParam(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FCActivityCategory(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IconType(&self) -> &ColumnData {
&self.columns[5]
}
}
