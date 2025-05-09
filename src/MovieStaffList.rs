#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MovieStaffList {
exd: EXD,
exh: EXH,
}
impl MovieStaffList {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MovieStaffList").unwrap();let exd = game_data.read_excel_sheet("MovieStaffList", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MovieStaffListRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MovieStaffListRow { columns: row.columns.clone() }
}
}
pub struct MovieStaffListRow {
columns: Vec<ColumnData>,
}
impl MovieStaffListRow {
pub fn StartTime(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EndTime(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
