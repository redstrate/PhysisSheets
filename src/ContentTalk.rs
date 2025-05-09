#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentTalk {
exd: EXD,
exh: EXH,
}
impl ContentTalk {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentTalk").unwrap();let exd = game_data.read_excel_sheet("ContentTalk", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentTalkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentTalkRow { columns: row.columns.clone() }
}
}
pub struct ContentTalkRow {
columns: Vec<ColumnData>,
}
impl ContentTalkRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ContentTalkParam(&self) -> &ColumnData {
&self.columns[1]
}
}
