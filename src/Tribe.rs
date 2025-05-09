#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Tribe {
exd: EXD,
exh: EXH,
}
impl Tribe {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Tribe").unwrap();let exd = game_data.read_excel_sheet("Tribe", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TribeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TribeRow { columns: row.columns.clone() }
}
}
pub struct TribeRow {
columns: Vec<ColumnData>,
}
impl TribeRow {
pub fn Masculine(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Feminine(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Hp(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Mp(&self) -> &ColumnData {
&self.columns[3]
}
pub fn STR(&self) -> &ColumnData {
&self.columns[4]
}
pub fn VIT(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DEX(&self) -> &ColumnData {
&self.columns[6]
}
pub fn INT(&self) -> &ColumnData {
&self.columns[7]
}
pub fn MND(&self) -> &ColumnData {
&self.columns[8]
}
pub fn PIE(&self) -> &ColumnData {
&self.columns[9]
}
}
