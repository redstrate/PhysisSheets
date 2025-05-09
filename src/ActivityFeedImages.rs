#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActivityFeedImages {
exd: EXD,
exh: EXH,
}
impl ActivityFeedImages {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActivityFeedImages").unwrap();let exd = game_data.read_excel_sheet("ActivityFeedImages", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActivityFeedImagesRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActivityFeedImagesRow { columns: row.columns.clone() }
}
}
pub struct ActivityFeedImagesRow {
columns: Vec<ColumnData>,
}
impl ActivityFeedImagesRow {
pub fn ExpansionImage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ActivityFeedJA(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ActivityFeedEN(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ActivityFeedDE(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ActivityFeedFR(&self) -> &ColumnData {
&self.columns[4]
}
}
