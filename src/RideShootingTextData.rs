#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RideShootingTextData {
exd: EXD,
exh: EXH,
}
impl RideShootingTextData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RideShootingTextData").unwrap();let exd = game_data.read_excel_sheet("RideShootingTextData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RideShootingTextDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RideShootingTextDataRow { columns: row.columns.clone() }
}
}
pub struct RideShootingTextDataRow {
columns: Vec<ColumnData>,
}
impl RideShootingTextDataRow {
pub fn String(&self) -> &ColumnData {
&self.columns[0]
}
}
