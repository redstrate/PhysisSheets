#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeapon5SpiritTalkType {
exd: EXD,
exh: EXH,
}
impl AnimaWeapon5SpiritTalkType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeapon5SpiritTalkType").unwrap();let exd = game_data.read_excel_sheet("AnimaWeapon5SpiritTalkType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeapon5SpiritTalkTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeapon5SpiritTalkTypeRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeapon5SpiritTalkTypeRow {
columns: Vec<ColumnData>,
}
impl AnimaWeapon5SpiritTalkTypeRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
