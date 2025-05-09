#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Role {
exd: EXD,
exh: EXH,
}
impl Role {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Role").unwrap();let exd = game_data.read_excel_sheet("Role", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RoleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RoleRow { columns: row.columns.clone() }
}
}
pub struct RoleRow {
columns: Vec<ColumnData>,
}
impl RoleRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
}
