#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Condition {
exd: EXD,
exh: EXH,
}
impl Condition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Condition").unwrap();let exd = game_data.read_excel_sheet("Condition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ConditionRow { columns: row.columns.clone() }
}
}
pub struct ConditionRow {
columns: Vec<ColumnData>,
}
impl ConditionRow {
pub fn LogMessage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Permission(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LogMessagePriority(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IsNetworked(&self) -> &ColumnData {
&self.columns[3]
}
}
