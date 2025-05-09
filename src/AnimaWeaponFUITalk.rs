#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeaponFUITalk {
exd: EXD,
exh: EXH,
}
impl AnimaWeaponFUITalk {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeaponFUITalk").unwrap();let exd = game_data.read_excel_sheet("AnimaWeaponFUITalk", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeaponFUITalkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeaponFUITalkRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeaponFUITalkRow {
columns: Vec<ColumnData>,
}
impl AnimaWeaponFUITalkRow {
pub fn Dialogue(&self) -> &ColumnData {
&self.columns[0]
}
}
