#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Stain {
exd: EXD,
exh: EXH,
}
impl Stain {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Stain").unwrap();let exd = game_data.read_excel_sheet("Stain", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> StainRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
StainRow { columns: row.columns.clone() }
}
}
pub struct StainRow {
columns: Vec<ColumnData>,
}
impl StainRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Name2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Color(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Shade(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SubOrder(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[6]
}
}
