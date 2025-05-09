#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentExAction {
exd: EXD,
exh: EXH,
}
impl ContentExAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentExAction").unwrap();let exd = game_data.read_excel_sheet("ContentExAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentExActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentExActionRow { columns: row.columns.clone() }
}
}
pub struct ContentExActionRow {
columns: Vec<ColumnData>,
}
impl ContentExActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Charges(&self) -> &ColumnData {
&self.columns[1]
}
}
