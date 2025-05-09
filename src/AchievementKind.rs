#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AchievementKind {
exd: EXD,
exh: EXH,
}
impl AchievementKind {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AchievementKind").unwrap();let exd = game_data.read_excel_sheet("AchievementKind", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AchievementKindRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AchievementKindRow { columns: row.columns.clone() }
}
}
pub struct AchievementKindRow {
columns: Vec<ColumnData>,
}
impl AchievementKindRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[1]
}
}
