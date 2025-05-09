#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ClassJobResident {
exd: EXD,
exh: EXH,
}
impl ClassJobResident {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ClassJobResident").unwrap();let exd = game_data.read_excel_sheet("ClassJobResident", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ClassJobResidentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ClassJobResidentRow { columns: row.columns.clone() }
}
}
pub struct ClassJobResidentRow {
columns: Vec<ColumnData>,
}
impl ClassJobResidentRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
