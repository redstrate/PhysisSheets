#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AnimaWeaponIcon {
exd: EXD,
exh: EXH,
}
impl AnimaWeaponIcon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeaponIcon").unwrap();let exd = game_data.read_excel_sheet("AnimaWeaponIcon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeaponIconRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
AnimaWeaponIconRow { columns }
}
}
pub struct AnimaWeaponIconRow {
columns: Vec<ColumnData>,
}
impl AnimaWeaponIconRow {
pub fn Hyperconductive(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Reborn(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Sharpened(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Zodiac(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ZodiacLux(&self) -> &ColumnData {
&self.columns[4]
}
}
