#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MoveVfx {
exd: EXD,
exh: EXH,
}
impl MoveVfx {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MoveVfx").unwrap();let exd = game_data.read_excel_sheet("MoveVfx", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MoveVfxRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MoveVfxRow { columns: row.columns.clone() }
}
}
pub struct MoveVfxRow {
columns: Vec<ColumnData>,
}
impl MoveVfxRow {
pub fn VFXNormal(&self) -> &ColumnData {
&self.columns[0]
}
pub fn VFXWalking(&self) -> &ColumnData {
&self.columns[1]
}
}
