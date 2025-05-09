#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MonsterNote {
exd: EXD,
exh: EXH,
}
impl MonsterNote {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MonsterNote").unwrap();let exd = game_data.read_excel_sheet("MonsterNote", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MonsterNoteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MonsterNoteRow { columns: row.columns.clone() }
}
}
pub struct MonsterNoteRow {
columns: Vec<ColumnData>,
}
impl MonsterNoteRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MonsterNoteTarget(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Count(&self) -> &ColumnData {
&self.columns[3]
}
}
