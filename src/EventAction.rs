#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventAction {
exd: EXD,
exh: EXH,
}
impl EventAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventAction").unwrap();let exd = game_data.read_excel_sheet("EventAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventActionRow { columns: row.columns.clone() }
}
}
pub struct EventActionRow {
columns: Vec<ColumnData>,
}
impl EventActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn StartAnimation(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LoopAnimation(&self) -> &ColumnData {
&self.columns[3]
}
pub fn EndAnimation(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CastTime(&self) -> &ColumnData {
&self.columns[5]
}
}
