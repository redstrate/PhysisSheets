#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIMinionPopAreas {
exd: EXD,
exh: EXH,
}
impl MJIMinionPopAreas {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIMinionPopAreas").unwrap();let exd = game_data.read_excel_sheet("MJIMinionPopAreas", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIMinionPopAreasRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIMinionPopAreasRow { columns: row.columns.clone() }
}
}
pub struct MJIMinionPopAreasRow {
columns: Vec<ColumnData>,
}
impl MJIMinionPopAreasRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn X(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredFunction(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[4]
}
}
