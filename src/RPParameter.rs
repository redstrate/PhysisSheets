#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RPParameter {
exd: EXD,
exh: EXH,
}
impl RPParameter {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RPParameter").unwrap();let exd = game_data.read_excel_sheet("RPParameter", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RPParameterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RPParameterRow { columns: row.columns.clone() }
}
}
pub struct RPParameterRow {
columns: Vec<ColumnData>,
}
impl RPParameterRow {
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Sex(&self) -> &ColumnData {
&self.columns[2]
}
}
