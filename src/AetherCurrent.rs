#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AetherCurrent {
exd: EXD,
exh: EXH,
}
impl AetherCurrent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AetherCurrent").unwrap();let exd = game_data.read_excel_sheet("AetherCurrent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AetherCurrentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AetherCurrentRow { columns: row.columns.clone() }
}
}
pub struct AetherCurrentRow {
columns: Vec<ColumnData>,
}
impl AetherCurrentRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
}
