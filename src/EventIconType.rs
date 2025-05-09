#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventIconType {
exd: EXD,
exh: EXH,
}
impl EventIconType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventIconType").unwrap();let exd = game_data.read_excel_sheet("EventIconType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventIconTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventIconTypeRow { columns: row.columns.clone() }
}
}
pub struct EventIconTypeRow {
columns: Vec<ColumnData>,
}
impl EventIconTypeRow {
pub fn NpcIconAvailable(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MapIconAvailable(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NpcIconInvalid(&self) -> &ColumnData {
&self.columns[2]
}
pub fn MapIconInvalid(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IconRange(&self) -> &ColumnData {
&self.columns[4]
}
}
