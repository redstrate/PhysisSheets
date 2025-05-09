#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct UIColor {
exd: EXD,
exh: EXH,
}
impl UIColor {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("UIColor").unwrap();let exd = game_data.read_excel_sheet("UIColor", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> UIColorRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
UIColorRow { columns: row.columns.clone() }
}
}
pub struct UIColorRow {
columns: Vec<ColumnData>,
}
impl UIColorRow {
pub fn Dark(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Light(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ClassicFF(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ClearBlue(&self) -> &ColumnData {
&self.columns[3]
}
}
