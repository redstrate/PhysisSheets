#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AozAction {
exd: EXD,
exh: EXH,
}
impl AozAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AozAction").unwrap();let exd = game_data.read_excel_sheet("AozAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AozActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AozActionRow { columns: row.columns.clone() }
}
}
pub struct AozActionRow {
columns: Vec<ColumnData>,
}
impl AozActionRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[1]
}
}
