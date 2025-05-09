#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaDungeonPortal {
exd: EXD,
exh: EXH,
}
impl EurekaDungeonPortal {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaDungeonPortal").unwrap();let exd = game_data.read_excel_sheet("EurekaDungeonPortal", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaDungeonPortalRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaDungeonPortalRow { columns: row.columns.clone() }
}
}
pub struct EurekaDungeonPortalRow {
columns: Vec<ColumnData>,
}
impl EurekaDungeonPortalRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
