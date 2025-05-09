#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DpsChallengeOfficer {
exd: EXD,
exh: EXH,
}
impl DpsChallengeOfficer {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DpsChallengeOfficer").unwrap();let exd = game_data.read_excel_sheet("DpsChallengeOfficer", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DpsChallengeOfficerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DpsChallengeOfficerRow { columns: row.columns.clone() }
}
}
pub struct DpsChallengeOfficerRow {
columns: Vec<ColumnData>,
}
impl DpsChallengeOfficerRow {
pub fn ChallengeName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockQuest(&self) -> &ColumnData {
&self.columns[1]
}
}
