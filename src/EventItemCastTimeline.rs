#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventItemCastTimeline {
exd: EXD,
exh: EXH,
}
impl EventItemCastTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventItemCastTimeline").unwrap();let exd = game_data.read_excel_sheet("EventItemCastTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventItemCastTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventItemCastTimelineRow { columns: row.columns.clone() }
}
}
pub struct EventItemCastTimelineRow {
columns: Vec<ColumnData>,
}
impl EventItemCastTimelineRow {
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[0]
}
}
