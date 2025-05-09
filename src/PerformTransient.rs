#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PerformTransient {
exd: EXD,
exh: EXH,
}
impl PerformTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PerformTransient").unwrap();let exd = game_data.read_excel_sheet("PerformTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PerformTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PerformTransientRow { columns: row.columns.clone() }
}
}
pub struct PerformTransientRow {
columns: Vec<ColumnData>,
}
impl PerformTransientRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
