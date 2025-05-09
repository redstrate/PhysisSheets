#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RecipeLookup {
exd: EXD,
exh: EXH,
}
impl RecipeLookup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RecipeLookup").unwrap();let exd = game_data.read_excel_sheet("RecipeLookup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RecipeLookupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RecipeLookupRow { columns: row.columns.clone() }
}
}
pub struct RecipeLookupRow {
columns: Vec<ColumnData>,
}
impl RecipeLookupRow {
pub fn CRP(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BSM(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ARM(&self) -> &ColumnData {
&self.columns[2]
}
pub fn GSM(&self) -> &ColumnData {
&self.columns[3]
}
pub fn LTW(&self) -> &ColumnData {
&self.columns[4]
}
pub fn WVR(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ALC(&self) -> &ColumnData {
&self.columns[6]
}
pub fn CUL(&self) -> &ColumnData {
&self.columns[7]
}
}
