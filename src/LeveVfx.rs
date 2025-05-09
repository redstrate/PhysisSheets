#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LeveVfx {
exd: EXD,
exh: EXH,
}
impl LeveVfx {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LeveVfx").unwrap();let exd = game_data.read_excel_sheet("LeveVfx", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LeveVfxRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LeveVfxRow { columns: row.columns.clone() }
}
}
pub struct LeveVfxRow {
columns: Vec<ColumnData>,
}
impl LeveVfxRow {
pub fn Effect(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
}
