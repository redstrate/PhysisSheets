#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DefaultTalkLipSyncType {
exd: EXD,
exh: EXH,
}
impl DefaultTalkLipSyncType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DefaultTalkLipSyncType").unwrap();let exd = game_data.read_excel_sheet("DefaultTalkLipSyncType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DefaultTalkLipSyncTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DefaultTalkLipSyncTypeRow { columns: row.columns.clone() }
}
}
pub struct DefaultTalkLipSyncTypeRow {
columns: Vec<ColumnData>,
}
impl DefaultTalkLipSyncTypeRow {
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[0]
}
}
