#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OrnamentTransient {
exd: EXD,
exh: EXH,
}
impl OrnamentTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OrnamentTransient").unwrap();let exd = game_data.read_excel_sheet("OrnamentTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrnamentTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OrnamentTransientRow { columns: row.columns.clone() }
}
}
pub struct OrnamentTransientRow {
columns: Vec<ColumnData>,
}
impl OrnamentTransientRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
