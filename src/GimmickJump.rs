#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GimmickJump {
exd: EXD,
exh: EXH,
}
impl GimmickJump {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GimmickJump").unwrap();let exd = game_data.read_excel_sheet("GimmickJump", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GimmickJumpRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GimmickJumpRow { columns: row.columns.clone() }
}
}
pub struct GimmickJumpRow {
columns: Vec<ColumnData>,
}
impl GimmickJumpRow {
pub fn LoopMotion(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EndMotion(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FallDamage(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn StartClient(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
}
