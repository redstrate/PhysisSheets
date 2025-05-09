#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionComboRoute {
exd: EXD,
exh: EXH,
}
impl ActionComboRoute {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionComboRoute").unwrap();let exd = game_data.read_excel_sheet("ActionComboRoute", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionComboRouteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionComboRouteRow { columns: row.columns.clone() }
}
}
pub struct ActionComboRouteRow {
columns: Vec<ColumnData>,
}
impl ActionComboRouteRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[3]
}
}
