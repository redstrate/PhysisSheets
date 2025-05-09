#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Calendar {
exd: EXD,
exh: EXH,
}
impl Calendar {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Calendar").unwrap();let exd = game_data.read_excel_sheet("Calendar", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CalendarRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CalendarRow { columns: row.columns.clone() }
}
}
pub struct CalendarRow {
columns: Vec<ColumnData>,
}
impl CalendarRow {
pub fn CalendarStruct(&self) -> &ColumnData {
&self.columns[0]
}
}
