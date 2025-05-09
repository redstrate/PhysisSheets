#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventSystemDefine {
exd: EXD,
exh: EXH,
}
impl EventSystemDefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventSystemDefine").unwrap();let exd = game_data.read_excel_sheet("EventSystemDefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventSystemDefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventSystemDefineRow { columns: row.columns.clone() }
}
}
pub struct EventSystemDefineRow {
columns: Vec<ColumnData>,
}
impl EventSystemDefineRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DefineValue(&self) -> &ColumnData {
&self.columns[1]
}
}
