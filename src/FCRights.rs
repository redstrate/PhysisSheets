#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCRights {
exd: EXD,
exh: EXH,
}
impl FCRights {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCRights").unwrap();let exd = game_data.read_excel_sheet("FCRights", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCRightsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCRightsRow { columns: row.columns.clone() }
}
}
pub struct FCRightsRow {
columns: Vec<ColumnData>,
}
impl FCRightsRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn FCRank(&self) -> &ColumnData {
&self.columns[3]
}
}
