#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct InstanceContentTextData {
exd: EXD,
exh: EXH,
}
impl InstanceContentTextData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("InstanceContentTextData").unwrap();let exd = game_data.read_excel_sheet("InstanceContentTextData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> InstanceContentTextDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
InstanceContentTextDataRow { columns: row.columns.clone() }
}
}
pub struct InstanceContentTextDataRow {
columns: Vec<ColumnData>,
}
impl InstanceContentTextDataRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
