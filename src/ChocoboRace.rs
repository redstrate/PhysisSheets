#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboRace {
exd: EXD,
exh: EXH,
}
impl ChocoboRace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboRace").unwrap();let exd = game_data.read_excel_sheet("ChocoboRace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboRaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboRaceRow { columns: row.columns.clone() }
}
}
pub struct ChocoboRaceRow {
columns: Vec<ColumnData>,
}
impl ChocoboRaceRow {
pub fn ChocoboRaceRank(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ChocoboRaceTerritory(&self) -> &ColumnData {
&self.columns[1]
}
}
