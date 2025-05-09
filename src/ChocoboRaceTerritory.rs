#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboRaceTerritory {
exd: EXD,
exh: EXH,
}
impl ChocoboRaceTerritory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboRaceTerritory").unwrap();let exd = game_data.read_excel_sheet("ChocoboRaceTerritory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboRaceTerritoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboRaceTerritoryRow { columns: row.columns.clone() }
}
}
pub struct ChocoboRaceTerritoryRow {
columns: Vec<ColumnData>,
}
impl ChocoboRaceTerritoryRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[1]
}
}
