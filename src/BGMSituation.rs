#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BGMSituation {
exd: EXD,
exh: EXH,
}
impl BGMSituation {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BGMSituation").unwrap();let exd = game_data.read_excel_sheet("BGMSituation", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BGMSituationRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BGMSituationRow { columns: row.columns.clone() }
}
}
pub struct BGMSituationRow {
columns: Vec<ColumnData>,
}
impl BGMSituationRow {
pub fn DaytimeID(&self) -> &ColumnData {
&self.columns[0]
}
pub fn NightID(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BattleID(&self) -> &ColumnData {
&self.columns[2]
}
pub fn DaybreakID(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TwilightID(&self) -> &ColumnData {
&self.columns[4]
}
}
