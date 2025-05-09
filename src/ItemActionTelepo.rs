#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemActionTelepo {
exd: EXD,
exh: EXH,
}
impl ItemActionTelepo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemActionTelepo").unwrap();let exd = game_data.read_excel_sheet("ItemActionTelepo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemActionTelepoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemActionTelepoRow { columns: row.columns.clone() }
}
}
pub struct ItemActionTelepoRow {
columns: Vec<ColumnData>,
}
impl ItemActionTelepoRow {
pub fn Requirement(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DenyMessage(&self) -> &ColumnData {
&self.columns[1]
}
}
