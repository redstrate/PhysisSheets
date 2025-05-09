#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionProcStatus {
exd: EXD,
exh: EXH,
}
impl ActionProcStatus {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionProcStatus").unwrap();let exd = game_data.read_excel_sheet("ActionProcStatus", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionProcStatusRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionProcStatusRow { columns: row.columns.clone() }
}
}
pub struct ActionProcStatusRow {
columns: Vec<ColumnData>,
}
impl ActionProcStatusRow {
pub fn Status(&self) -> &ColumnData {
&self.columns[0]
}
}
