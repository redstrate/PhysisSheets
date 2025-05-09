#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ChocoboRaceStatus {
exd: EXD,
exh: EXH,
}
impl ChocoboRaceStatus {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ChocoboRaceStatus").unwrap();let exd = game_data.read_excel_sheet("ChocoboRaceStatus", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ChocoboRaceStatusRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ChocoboRaceStatusRow { columns: row.columns.clone() }
}
}
pub struct ChocoboRaceStatusRow {
columns: Vec<ColumnData>,
}
impl ChocoboRaceStatusRow {
pub fn Status(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
