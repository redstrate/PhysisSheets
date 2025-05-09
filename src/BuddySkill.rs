#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BuddySkill {
exd: EXD,
exh: EXH,
}
impl BuddySkill {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BuddySkill").unwrap();let exd = game_data.read_excel_sheet("BuddySkill", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BuddySkillRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BuddySkillRow { columns: row.columns.clone() }
}
}
pub struct BuddySkillRow {
columns: Vec<ColumnData>,
}
impl BuddySkillRow {
pub fn Defender(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Attacker(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Healer(&self) -> &ColumnData {
&self.columns[2]
}
pub fn BuddyLevel(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IsActive(&self) -> &ColumnData {
&self.columns[4]
}
}
