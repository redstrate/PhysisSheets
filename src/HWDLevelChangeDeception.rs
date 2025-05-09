#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDLevelChangeDeception {
exd: EXD,
exh: EXH,
}
impl HWDLevelChangeDeception {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDLevelChangeDeception").unwrap();let exd = game_data.read_excel_sheet("HWDLevelChangeDeception", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDLevelChangeDeceptionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDLevelChangeDeceptionRow { columns: row.columns.clone() }
}
}
pub struct HWDLevelChangeDeceptionRow {
columns: Vec<ColumnData>,
}
impl HWDLevelChangeDeceptionRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
}
