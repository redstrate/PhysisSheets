#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeaponItem {
exd: EXD,
exh: EXH,
}
impl AnimaWeaponItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeaponItem").unwrap();let exd = game_data.read_excel_sheet("AnimaWeaponItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeaponItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeaponItemRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeaponItemRow {
columns: Vec<ColumnData>,
}
impl AnimaWeaponItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}
