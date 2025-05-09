#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AetherialWheel {
exd: EXD,
exh: EXH,
}
impl AetherialWheel {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AetherialWheel").unwrap();let exd = game_data.read_excel_sheet("AetherialWheel", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AetherialWheelRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AetherialWheelRow { columns: row.columns.clone() }
}
}
pub struct AetherialWheelRow {
columns: Vec<ColumnData>,
}
impl AetherialWheelRow {
pub fn ItemUnprimed(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemPrimed(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Grade(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HoursRequired(&self) -> &ColumnData {
&self.columns[3]
}
}
