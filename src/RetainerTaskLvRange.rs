#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RetainerTaskLvRange {
exd: EXD,
exh: EXH,
}
impl RetainerTaskLvRange {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RetainerTaskLvRange").unwrap();let exd = game_data.read_excel_sheet("RetainerTaskLvRange", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RetainerTaskLvRangeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RetainerTaskLvRangeRow { columns: row.columns.clone() }
}
}
pub struct RetainerTaskLvRangeRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskLvRangeRow {
pub fn Min(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Max(&self) -> &ColumnData {
&self.columns[1]
}
}
