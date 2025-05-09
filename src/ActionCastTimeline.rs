#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionCastTimeline {
exd: EXD,
exh: EXH,
}
impl ActionCastTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionCastTimeline").unwrap();let exd = game_data.read_excel_sheet("ActionCastTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionCastTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionCastTimelineRow { columns: row.columns.clone() }
}
}
pub struct ActionCastTimelineRow {
columns: Vec<ColumnData>,
}
impl ActionCastTimelineRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn VFX(&self) -> &ColumnData {
&self.columns[1]
}
}
