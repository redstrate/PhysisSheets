#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DynamicEventType {
exd: EXD,
exh: EXH,
}
impl DynamicEventType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DynamicEventType").unwrap();let exd = game_data.read_excel_sheet("DynamicEventType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DynamicEventTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DynamicEventTypeRow { columns: row.columns.clone() }
}
}
pub struct DynamicEventTypeRow {
columns: Vec<ColumnData>,
}
impl DynamicEventTypeRow {
pub fn IconObjective0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn IconObjective1(&self) -> &ColumnData {
&self.columns[1]
}
}
