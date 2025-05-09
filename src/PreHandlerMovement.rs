#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PreHandlerMovement {
exd: EXD,
exh: EXH,
}
impl PreHandlerMovement {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PreHandlerMovement").unwrap();let exd = game_data.read_excel_sheet("PreHandlerMovement", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PreHandlerMovementRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PreHandlerMovementRow { columns: row.columns.clone() }
}
}
pub struct PreHandlerMovementRow {
columns: Vec<ColumnData>,
}
impl PreHandlerMovementRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}
