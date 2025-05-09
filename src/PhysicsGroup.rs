#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PhysicsGroup {
exd: EXD,
exh: EXH,
}
impl PhysicsGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PhysicsGroup").unwrap();let exd = game_data.read_excel_sheet("PhysicsGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PhysicsGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PhysicsGroupRow { columns: row.columns.clone() }
}
}
pub struct PhysicsGroupRow {
columns: Vec<ColumnData>,
}
impl PhysicsGroupRow {
pub fn SimulationTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PS3SimulationTime(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RootFollowingGame(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RootFollowingCutScene(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ConfigSwitch(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ResetByLookAt(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ForceAttractByPhysicsOff(&self) -> &ColumnData {
&self.columns[6]
}
}
