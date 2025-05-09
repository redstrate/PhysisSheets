#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventIconPriorityPair {
exd: EXD,
exh: EXH,
}
impl EventIconPriorityPair {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventIconPriorityPair").unwrap();let exd = game_data.read_excel_sheet("EventIconPriorityPair", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventIconPriorityPairRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventIconPriorityPairRow { columns: row.columns.clone() }
}
}
pub struct EventIconPriorityPairRow {
columns: Vec<ColumnData>,
}
impl EventIconPriorityPairRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
}
