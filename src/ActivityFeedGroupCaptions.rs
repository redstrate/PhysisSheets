#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActivityFeedGroupCaptions {
exd: EXD,
exh: EXH,
}
impl ActivityFeedGroupCaptions {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActivityFeedGroupCaptions").unwrap();let exd = game_data.read_excel_sheet("ActivityFeedGroupCaptions", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActivityFeedGroupCaptionsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActivityFeedGroupCaptionsRow { columns: row.columns.clone() }
}
}
pub struct ActivityFeedGroupCaptionsRow {
columns: Vec<ColumnData>,
}
impl ActivityFeedGroupCaptionsRow {
pub fn JA(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EN(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DE(&self) -> &ColumnData {
&self.columns[2]
}
pub fn FR(&self) -> &ColumnData {
&self.columns[3]
}
}
