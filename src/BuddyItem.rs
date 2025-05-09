#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BuddyItem {
exd: EXD,
exh: EXH,
}
impl BuddyItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BuddyItem").unwrap();let exd = game_data.read_excel_sheet("BuddyItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BuddyItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BuddyItemRow { columns: row.columns.clone() }
}
}
pub struct BuddyItemRow {
columns: Vec<ColumnData>,
}
impl BuddyItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Status(&self) -> &ColumnData {
&self.columns[1]
}
pub fn UseField(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UseTraining(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
}
