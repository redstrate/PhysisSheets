#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AchievementHideCondition {
exd: EXD,
exh: EXH,
}
impl AchievementHideCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AchievementHideCondition").unwrap();let exd = game_data.read_excel_sheet("AchievementHideCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AchievementHideConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AchievementHideConditionRow { columns: row.columns.clone() }
}
}
pub struct AchievementHideConditionRow {
columns: Vec<ColumnData>,
}
impl AchievementHideConditionRow {
pub fn HideAchievement(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HideName(&self) -> &ColumnData {
&self.columns[1]
}
pub fn HideConditions(&self) -> &ColumnData {
&self.columns[2]
}
}
