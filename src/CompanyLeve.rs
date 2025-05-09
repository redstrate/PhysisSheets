#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyLeve {
exd: EXD,
exh: EXH,
}
impl CompanyLeve {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyLeve").unwrap();let exd = game_data.read_excel_sheet("CompanyLeve", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyLeveRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyLeveRow { columns: row.columns.clone() }
}
}
pub struct CompanyLeveRow {
columns: Vec<ColumnData>,
}
impl CompanyLeveRow {
pub fn RoutePointTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CompanyLeveStruct(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ToDoSequence(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RuleParam(&self) -> &ColumnData {
&self.columns[4]
}
}
