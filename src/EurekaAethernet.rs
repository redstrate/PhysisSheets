#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaAethernet {
exd: EXD,
exh: EXH,
}
impl EurekaAethernet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaAethernet").unwrap();let exd = game_data.read_excel_sheet("EurekaAethernet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaAethernetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaAethernetRow { columns: row.columns.clone() }
}
}
pub struct EurekaAethernetRow {
columns: Vec<ColumnData>,
}
impl EurekaAethernetRow {
pub fn Location(&self) -> &ColumnData {
&self.columns[0]
}
}
