#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentNpcTalk {
exd: EXD,
exh: EXH,
}
impl ContentNpcTalk {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentNpcTalk").unwrap();let exd = game_data.read_excel_sheet("ContentNpcTalk", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentNpcTalkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentNpcTalkRow { columns: row.columns.clone() }
}
}
pub struct ContentNpcTalkRow {
columns: Vec<ColumnData>,
}
impl ContentNpcTalkRow {
pub fn ContentTalk(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[1]
}
}
