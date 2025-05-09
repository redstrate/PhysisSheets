#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HowToCategory {
exd: EXD,
exh: EXH,
}
impl HowToCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HowToCategory").unwrap();let exd = game_data.read_excel_sheet("HowToCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HowToCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HowToCategoryRow { columns: row.columns.clone() }
}
}
pub struct HowToCategoryRow {
columns: Vec<ColumnData>,
}
impl HowToCategoryRow {
pub fn Category(&self) -> &ColumnData {
&self.columns[0]
}
}
