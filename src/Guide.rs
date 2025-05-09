#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Guide {
exd: EXD,
exh: EXH,
}
impl Guide {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Guide").unwrap();let exd = game_data.read_excel_sheet("Guide", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GuideRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GuideRow { columns: row.columns.clone() }
}
}
pub struct GuideRow {
columns: Vec<ColumnData>,
}
impl GuideRow {
pub fn GuideTitle(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GuidePage(&self) -> &ColumnData {
&self.columns[1]
}
}
