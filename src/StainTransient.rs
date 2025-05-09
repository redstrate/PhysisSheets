#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct StainTransient {
exd: EXD,
exh: EXH,
}
impl StainTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("StainTransient").unwrap();let exd = game_data.read_excel_sheet("StainTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> StainTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
StainTransientRow { columns: row.columns.clone() }
}
}
pub struct StainTransientRow {
columns: Vec<ColumnData>,
}
impl StainTransientRow {
pub fn Item1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item2(&self) -> &ColumnData {
&self.columns[1]
}
}
