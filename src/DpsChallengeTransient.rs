#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DpsChallengeTransient {
exd: EXD,
exh: EXH,
}
impl DpsChallengeTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DpsChallengeTransient").unwrap();let exd = game_data.read_excel_sheet("DpsChallengeTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DpsChallengeTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DpsChallengeTransientRow { columns: row.columns.clone() }
}
}
pub struct DpsChallengeTransientRow {
columns: Vec<ColumnData>,
}
impl DpsChallengeTransientRow {
pub fn InstanceContent(&self) -> &ColumnData {
&self.columns[0]
}
}
