#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboRaceAbilityType {
exd: EXD,
exh: EXH,
}
impl ChocoboRaceAbilityType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboRaceAbilityType").unwrap();let exd = game_data.read_excel_sheet("ChocoboRaceAbilityType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboRaceAbilityTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboRaceAbilityTypeRow { columns: row.columns.clone() }
}
}
pub struct ChocoboRaceAbilityTypeRow {
columns: Vec<ColumnData>,
}
impl ChocoboRaceAbilityTypeRow {
pub fn IsActive(&self) -> &ColumnData {
&self.columns[0]
}
}
