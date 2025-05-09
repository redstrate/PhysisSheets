#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct InstanceContentBuff {
exd: EXD,
exh: EXH,
}
impl InstanceContentBuff {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("InstanceContentBuff").unwrap();let exd = game_data.read_excel_sheet("InstanceContentBuff", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> InstanceContentBuffRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
InstanceContentBuffRow { columns: row.columns.clone() }
}
}
pub struct InstanceContentBuffRow {
columns: Vec<ColumnData>,
}
impl InstanceContentBuffRow {
pub fn EchoStart(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EchoDeath(&self) -> &ColumnData {
&self.columns[1]
}
}
