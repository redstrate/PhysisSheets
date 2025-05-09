#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct UDS_Event {
exd: EXD,
exh: EXH,
}
impl UDS_Event {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("UDS_Event").unwrap();let exd = game_data.read_excel_sheet("UDS_Event", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> UDS_EventRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
UDS_EventRow { columns: row.columns.clone() }
}
}
pub struct UDS_EventRow {
columns: Vec<ColumnData>,
}
impl UDS_EventRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Property(&self) -> &ColumnData {
&self.columns[2]
}
}
