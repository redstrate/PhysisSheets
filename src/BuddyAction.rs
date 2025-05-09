#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BuddyAction {
exd: EXD,
exh: EXH,
}
impl BuddyAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BuddyAction").unwrap();let exd = game_data.read_excel_sheet("BuddyAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BuddyActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BuddyActionRow { columns: row.columns.clone() }
}
}
pub struct BuddyActionRow {
columns: Vec<ColumnData>,
}
impl BuddyActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IconStatus(&self) -> &ColumnData {
&self.columns[3]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Sort(&self) -> &ColumnData {
&self.columns[5]
}
}
