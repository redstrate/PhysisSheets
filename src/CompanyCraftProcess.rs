#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftProcess {
exd: EXD,
exh: EXH,
}
impl CompanyCraftProcess {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftProcess").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftProcess", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftProcessRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftProcessRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftProcessRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftProcessRow {
pub fn SupplyItem(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SetQuantity(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SetsRequired(&self) -> &ColumnData {
&self.columns[2]
}
}
