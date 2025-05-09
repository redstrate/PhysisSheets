#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct VFX {
exd: EXD,
exh: EXH,
}
impl VFX {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("VFX").unwrap();let exd = game_data.read_excel_sheet("VFX", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> VFXRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
VFXRow { columns: row.columns.clone() }
}
}
pub struct VFXRow {
columns: Vec<ColumnData>,
}
impl VFXRow {
pub fn Location(&self) -> &ColumnData {
&self.columns[0]
}
}
