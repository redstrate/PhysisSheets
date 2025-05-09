#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PartyContentCutscene {
exd: EXD,
exh: EXH,
}
impl PartyContentCutscene {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PartyContentCutscene").unwrap();let exd = game_data.read_excel_sheet("PartyContentCutscene", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PartyContentCutsceneRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PartyContentCutsceneRow { columns: row.columns.clone() }
}
}
pub struct PartyContentCutsceneRow {
columns: Vec<ColumnData>,
}
impl PartyContentCutsceneRow {
pub fn Cutscene(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
