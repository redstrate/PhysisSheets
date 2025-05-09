#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesRefine {
exd: EXD,
exh: EXH,
}
impl CollectablesRefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesRefine").unwrap();let exd = game_data.read_excel_sheet("CollectablesRefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesRefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesRefineRow { columns: row.columns.clone() }
}
}
pub struct CollectablesRefineRow {
columns: Vec<ColumnData>,
}
impl CollectablesRefineRow {
pub fn CollectabilityLow(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CollectabilityMid(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CollectabilityHigh(&self) -> &ColumnData {
&self.columns[2]
}
}
