#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeaponFUITalkParam {
exd: EXD,
exh: EXH,
}
impl AnimaWeaponFUITalkParam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeaponFUITalkParam").unwrap();let exd = game_data.read_excel_sheet("AnimaWeaponFUITalkParam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeaponFUITalkParamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeaponFUITalkParamRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeaponFUITalkParamRow {
columns: Vec<ColumnData>,
}
impl AnimaWeaponFUITalkParamRow {
pub fn Prologue(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Epilogue(&self) -> &ColumnData {
&self.columns[1]
}
}
