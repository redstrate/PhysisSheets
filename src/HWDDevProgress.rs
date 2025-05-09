#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDDevProgress {
exd: EXD,
exh: EXH,
}
impl HWDDevProgress {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDDevProgress").unwrap();let exd = game_data.read_excel_sheet("HWDDevProgress", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDDevProgressRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDDevProgressRow { columns: row.columns.clone() }
}
}
pub struct HWDDevProgressRow {
columns: Vec<ColumnData>,
}
impl HWDDevProgressRow {
pub fn CanGoNext(&self) -> &ColumnData {
&self.columns[0]
}
}
