#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CustomTalkNestHandlers {
exd: EXD,
exh: EXH,
}
impl CustomTalkNestHandlers {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CustomTalkNestHandlers").unwrap();let exd = game_data.read_excel_sheet("CustomTalkNestHandlers", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CustomTalkNestHandlersRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CustomTalkNestHandlersRow { columns: row.columns.clone() }
}
}
pub struct CustomTalkNestHandlersRow {
columns: Vec<ColumnData>,
}
impl CustomTalkNestHandlersRow {
pub fn NestHandler(&self) -> &ColumnData {
&self.columns[0]
}
}
