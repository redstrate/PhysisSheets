#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSAchievementDailyDefine {
exd: EXD,
exh: EXH,
}
impl WKSAchievementDailyDefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSAchievementDailyDefine").unwrap();let exd = game_data.read_excel_sheet("WKSAchievementDailyDefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSAchievementDailyDefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSAchievementDailyDefineRow { columns: row.columns.clone() }
}
}
pub struct WKSAchievementDailyDefineRow {
columns: Vec<ColumnData>,
}
impl WKSAchievementDailyDefineRow {
pub fn RewardItem(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SuccessPointsRequired(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RewardQuantity(&self) -> &ColumnData {
&self.columns[2]
}
}
