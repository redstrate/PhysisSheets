#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeeklyBingoRewardData {
exd: EXD,
exh: EXH,
}
impl WeeklyBingoRewardData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeeklyBingoRewardData").unwrap();let exd = game_data.read_excel_sheet("WeeklyBingoRewardData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeeklyBingoRewardDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeeklyBingoRewardDataRow { columns: row.columns.clone() }
}
}
pub struct WeeklyBingoRewardDataRow {
columns: Vec<ColumnData>,
}
impl WeeklyBingoRewardDataRow {
pub fn RewardItem1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RewardItem2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RewardItem3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RewardQuantity1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RewardQuantity2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RewardQuantity3(&self) -> &ColumnData {
&self.columns[5]
}
pub fn RewardType1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn RewardType2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RewardType3(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RewardHq2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn RewardHq3(&self) -> &ColumnData {
&self.columns[11]
}
pub fn RewardHq1(&self) -> &ColumnData {
&self.columns[12]
}
}
