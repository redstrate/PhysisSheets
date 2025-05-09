#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeapon5 {
exd: EXD,
exh: EXH,
}
impl AnimaWeapon5 {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeapon5").unwrap();let exd = game_data.read_excel_sheet("AnimaWeapon5", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeapon5Row {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeapon5Row { columns: row.columns.clone() }
}
}
pub struct AnimaWeapon5Row {
columns: Vec<ColumnData>,
}
impl AnimaWeapon5Row {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SecondaryStatTotal(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Parameter(&self) -> &ColumnData {
&self.columns[3]
}
}
