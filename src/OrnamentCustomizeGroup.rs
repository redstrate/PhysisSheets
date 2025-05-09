#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OrnamentCustomizeGroup {
exd: EXD,
exh: EXH,
}
impl OrnamentCustomizeGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OrnamentCustomizeGroup").unwrap();let exd = game_data.read_excel_sheet("OrnamentCustomizeGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrnamentCustomizeGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OrnamentCustomizeGroupRow { columns: row.columns.clone() }
}
}
pub struct OrnamentCustomizeGroupRow {
columns: Vec<ColumnData>,
}
impl OrnamentCustomizeGroupRow {
pub fn Customize(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[3]
}
}
