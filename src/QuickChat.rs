#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuickChat {
exd: EXD,
exh: EXH,
}
impl QuickChat {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuickChat").unwrap();let exd = game_data.read_excel_sheet("QuickChat", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuickChatRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuickChatRow { columns: row.columns.clone() }
}
}
pub struct QuickChatRow {
columns: Vec<ColumnData>,
}
impl QuickChatRow {
pub fn NameAction(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Addon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn QuickChatTransient(&self) -> &ColumnData {
&self.columns[4]
}
}
