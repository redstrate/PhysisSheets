#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Title {
exd: EXD,
exh: EXH,
}
impl Title {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Title").unwrap();let exd = game_data.read_excel_sheet("Title", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TitleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TitleRow { columns: row.columns.clone() }
}
}
pub struct TitleRow {
columns: Vec<ColumnData>,
}
impl TitleRow {
pub fn Masculine(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Feminine(&self) -> &ColumnData {
&self.columns[1]
}
pub fn IsPrefix(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[3]
}
}
