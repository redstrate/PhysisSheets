#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentRewardCondition {
exd: EXD,
exh: EXH,
}
impl ContentRewardCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentRewardCondition").unwrap();let exd = game_data.read_excel_sheet("ContentRewardCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentRewardConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentRewardConditionRow { columns: row.columns.clone() }
}
}
pub struct ContentRewardConditionRow {
columns: Vec<ColumnData>,
}
impl ContentRewardConditionRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
