#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct YKW {
exd: EXD,
exh: EXH,
}
impl YKW {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("YKW").unwrap();let exd = game_data.read_excel_sheet("YKW", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> YKWRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
YKWRow { columns: row.columns.clone() }
}
}
pub struct YKWRow {
columns: Vec<ColumnData>,
}
impl YKWRow {
pub fn Transient(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Companion(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[3]
}
}
