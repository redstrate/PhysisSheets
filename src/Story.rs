#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Story {
exd: EXD,
exh: EXH,
}
impl Story {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Story").unwrap();let exd = game_data.read_excel_sheet("Story", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> StoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
StoryRow { columns: row.columns.clone() }
}
}
pub struct StoryRow {
columns: Vec<ColumnData>,
}
impl StoryRow {
pub fn StoryParams(&self) -> &ColumnData {
&self.columns[0]
}
pub fn StoryDefine(&self) -> &ColumnData {
&self.columns[1]
}
pub fn StoryListener(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Script(&self) -> &ColumnData {
&self.columns[3]
}
pub fn LayerSetTerritoryType(&self) -> &ColumnData {
&self.columns[4]
}
}
