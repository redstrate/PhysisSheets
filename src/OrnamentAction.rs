#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct OrnamentAction {
exd: EXD,
exh: EXH,
}
impl OrnamentAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("OrnamentAction").unwrap();let exd = game_data.read_excel_sheet("OrnamentAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrnamentActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OrnamentActionRow { columns: row.columns.clone() }
}
}
pub struct OrnamentActionRow {
columns: Vec<ColumnData>,
}
impl OrnamentActionRow {
pub fn Actions(&self) -> &ColumnData {
&self.columns[0]
}
}
