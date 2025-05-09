#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActivityFeedButtons {
exd: EXD,
exh: EXH,
}
impl ActivityFeedButtons {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActivityFeedButtons").unwrap();let exd = game_data.read_excel_sheet("ActivityFeedButtons", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActivityFeedButtonsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActivityFeedButtonsRow { columns: row.columns.clone() }
}
}
pub struct ActivityFeedButtonsRow {
columns: Vec<ColumnData>,
}
impl ActivityFeedButtonsRow {
pub fn BannerURL(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Language(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PictureURL(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
}
