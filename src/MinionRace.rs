#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MinionRace {
exd: EXD,
exh: EXH,
}
impl MinionRace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MinionRace").unwrap();let exd = game_data.read_excel_sheet("MinionRace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MinionRaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MinionRaceRow { columns: row.columns.clone() }
}
}
pub struct MinionRaceRow {
columns: Vec<ColumnData>,
}
impl MinionRaceRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
