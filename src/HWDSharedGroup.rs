#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDSharedGroup {
exd: EXD,
exh: EXH,
}
impl HWDSharedGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDSharedGroup").unwrap();let exd = game_data.read_excel_sheet("HWDSharedGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDSharedGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDSharedGroupRow { columns: row.columns.clone() }
}
}
pub struct HWDSharedGroupRow {
columns: Vec<ColumnData>,
}
impl HWDSharedGroupRow {
pub fn LGBSharedGroup(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Param(&self) -> &ColumnData {
&self.columns[1]
}
}
