#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MountFlyingCondition {
exd: EXD,
exh: EXH,
}
impl MountFlyingCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MountFlyingCondition").unwrap();let exd = game_data.read_excel_sheet("MountFlyingCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MountFlyingConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MountFlyingConditionRow { columns: row.columns.clone() }
}
}
pub struct MountFlyingConditionRow {
columns: Vec<ColumnData>,
}
impl MountFlyingConditionRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
}
