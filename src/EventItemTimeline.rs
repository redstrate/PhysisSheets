#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventItemTimeline {
exd: EXD,
exh: EXH,
}
impl EventItemTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventItemTimeline").unwrap();let exd = game_data.read_excel_sheet("EventItemTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventItemTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventItemTimelineRow { columns: row.columns.clone() }
}
}
pub struct EventItemTimelineRow {
columns: Vec<ColumnData>,
}
impl EventItemTimelineRow {
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[0]
}
}
