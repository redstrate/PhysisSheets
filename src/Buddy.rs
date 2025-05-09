#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Buddy {
exd: EXD,
exh: EXH,
}
impl Buddy {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Buddy").unwrap();let exd = game_data.read_excel_sheet("Buddy", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BuddyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BuddyRow { columns: row.columns.clone() }
}
}
pub struct BuddyRow {
columns: Vec<ColumnData>,
}
impl BuddyRow {
pub fn SoundEffect4(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SoundEffect3(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SoundEffect2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SoundEffect1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn QuestRequirement2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn QuestRequirement1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn BaseEquip(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Base(&self) -> &ColumnData {
&self.columns[7]
}
}
