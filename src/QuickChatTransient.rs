#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuickChatTransient {
exd: EXD,
exh: EXH,
}
impl QuickChatTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuickChatTransient").unwrap();let exd = game_data.read_excel_sheet("QuickChatTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuickChatTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuickChatTransientRow { columns: row.columns.clone() }
}
}
pub struct QuickChatTransientRow {
columns: Vec<ColumnData>,
}
impl QuickChatTransientRow {
pub fn TextOutput(&self) -> &ColumnData {
&self.columns[0]
}
}
