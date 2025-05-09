#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboTaxiStand {
exd: EXD,
exh: EXH,
}
impl ChocoboTaxiStand {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboTaxiStand").unwrap();let exd = game_data.read_excel_sheet("ChocoboTaxiStand", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboTaxiStandRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboTaxiStandRow { columns: row.columns.clone() }
}
}
pub struct ChocoboTaxiStandRow {
columns: Vec<ColumnData>,
}
impl ChocoboTaxiStandRow {
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn TargetLocations(&self) -> &ColumnData {
&self.columns[1]
}
}
