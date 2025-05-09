#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventIconPriority {
exd: EXD,
exh: EXH,
}
impl EventIconPriority {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventIconPriority").unwrap();let exd = game_data.read_excel_sheet("EventIconPriority", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventIconPriorityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventIconPriorityRow { columns: row.columns.clone() }
}
}
pub struct EventIconPriorityRow {
columns: Vec<ColumnData>,
}
impl EventIconPriorityRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
}
