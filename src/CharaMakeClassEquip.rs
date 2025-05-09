#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaMakeClassEquip {
exd: EXD,
exh: EXH,
}
impl CharaMakeClassEquip {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaMakeClassEquip").unwrap();let exd = game_data.read_excel_sheet("CharaMakeClassEquip", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaMakeClassEquipRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaMakeClassEquipRow { columns: row.columns.clone() }
}
}
pub struct CharaMakeClassEquipRow {
columns: Vec<ColumnData>,
}
impl CharaMakeClassEquipRow {
pub fn Helmet(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Top(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Glove(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Down(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Shoes(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Weapon(&self) -> &ColumnData {
&self.columns[5]
}
pub fn SubWeapon(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Class(&self) -> &ColumnData {
&self.columns[7]
}
}
