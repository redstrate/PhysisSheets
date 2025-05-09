#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeapon5SpiritTalk {
exd: EXD,
exh: EXH,
}
impl AnimaWeapon5SpiritTalk {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeapon5SpiritTalk").unwrap();let exd = game_data.read_excel_sheet("AnimaWeapon5SpiritTalk", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeapon5SpiritTalkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeapon5SpiritTalkRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeapon5SpiritTalkRow {
columns: Vec<ColumnData>,
}
impl AnimaWeapon5SpiritTalkRow {
pub fn Dialogue(&self) -> &ColumnData {
&self.columns[0]
}
}
