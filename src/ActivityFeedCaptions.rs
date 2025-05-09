#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActivityFeedCaptions {
exd: EXD,
exh: EXH,
}
impl ActivityFeedCaptions {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActivityFeedCaptions").unwrap();let exd = game_data.read_excel_sheet("ActivityFeedCaptions", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActivityFeedCaptionsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActivityFeedCaptionsRow { columns: row.columns.clone() }
}
}
pub struct ActivityFeedCaptionsRow {
columns: Vec<ColumnData>,
}
impl ActivityFeedCaptionsRow {
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
