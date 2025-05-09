#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaMakeType {
exd: EXD,
exh: EXH,
}
impl CharaMakeType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaMakeType").unwrap();let exd = game_data.read_excel_sheet("CharaMakeType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaMakeTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaMakeTypeRow { columns: row.columns.clone() }
}
}
pub struct CharaMakeTypeRow {
columns: Vec<ColumnData>,
}
impl CharaMakeTypeRow {
pub fn CharaMakeStruct(&self) -> &ColumnData {
&self.columns[0]
}
pub fn VoiceStruct(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FacialFeatureOption(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Equipment(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[6]
}
}
