#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionTimelineReplace {
exd: EXD,
exh: EXH,
}
impl ActionTimelineReplace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionTimelineReplace").unwrap();let exd = game_data.read_excel_sheet("ActionTimelineReplace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionTimelineReplaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionTimelineReplaceRow { columns: row.columns.clone() }
}
}
pub struct ActionTimelineReplaceRow {
columns: Vec<ColumnData>,
}
impl ActionTimelineReplaceRow {
pub fn Old(&self) -> &ColumnData {
&self.columns[0]
}
pub fn New(&self) -> &ColumnData {
&self.columns[1]
}
}
