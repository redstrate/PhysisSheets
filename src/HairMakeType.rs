#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HairMakeType {
exd: EXD,
exh: EXH,
}
impl HairMakeType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HairMakeType").unwrap();let exd = game_data.read_excel_sheet("HairMakeType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HairMakeTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HairMakeTypeRow { columns: row.columns.clone() }
}
}
pub struct HairMakeTypeRow {
columns: Vec<ColumnData>,
}
impl HairMakeTypeRow {
pub fn CharaMakeStruct(&self) -> &ColumnData {
&self.columns[0]
}
pub fn FacialFeatureOption(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[4]
}
}
