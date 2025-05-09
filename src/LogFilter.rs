#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LogFilter {
exd: EXD,
exh: EXH,
}
impl LogFilter {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LogFilter").unwrap();let exd = game_data.read_excel_sheet("LogFilter", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LogFilterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LogFilterRow { columns: row.columns.clone() }
}
}
pub struct LogFilterRow {
columns: Vec<ColumnData>,
}
impl LogFilterRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Example(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Caster(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Target(&self) -> &ColumnData {
&self.columns[3]
}
pub fn LogKind(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DisplayOrder(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Preset(&self) -> &ColumnData {
&self.columns[7]
}
}
