#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CSBonusMission {
exd: EXD,
exh: EXH,
}
impl CSBonusMission {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusMission").unwrap();let exd = game_data.read_excel_sheet("CSBonusMission", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusMissionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CSBonusMissionRow { columns: row.columns.clone() }
}
}
pub struct CSBonusMissionRow {
columns: Vec<ColumnData>,
}
impl CSBonusMissionRow {
pub fn Content0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Content1(&self) -> &ColumnData {
&self.columns[1]
}
}
