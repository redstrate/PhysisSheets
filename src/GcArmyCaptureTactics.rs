#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyCaptureTactics {
exd: EXD,
exh: EXH,
}
impl GcArmyCaptureTactics {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyCaptureTactics").unwrap();let exd = game_data.read_excel_sheet("GcArmyCaptureTactics", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyCaptureTacticsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyCaptureTacticsRow { columns: row.columns.clone() }
}
}
pub struct GcArmyCaptureTacticsRow {
columns: Vec<ColumnData>,
}
impl GcArmyCaptureTacticsRow {
pub fn Tactic(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HP(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DamageDealt(&self) -> &ColumnData {
&self.columns[4]
}
pub fn DamageReceived(&self) -> &ColumnData {
&self.columns[5]
}
}
