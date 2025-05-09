#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AchievementCategory {
exd: EXD,
exh: EXH,
}
impl AchievementCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AchievementCategory").unwrap();let exd = game_data.read_excel_sheet("AchievementCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AchievementCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AchievementCategoryRow { columns: row.columns.clone() }
}
}
pub struct AchievementCategoryRow {
columns: Vec<ColumnData>,
}
impl AchievementCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AchievementKind(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ShowComplete(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HideCategory(&self) -> &ColumnData {
&self.columns[4]
}
}
