#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeeklyLotBonus {
exd: EXD,
exh: EXH,
}
impl WeeklyLotBonus {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeeklyLotBonus").unwrap();let exd = game_data.read_excel_sheet("WeeklyLotBonus", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeeklyLotBonusRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeeklyLotBonusRow { columns: row.columns.clone() }
}
}
pub struct WeeklyLotBonusRow {
columns: Vec<ColumnData>,
}
impl WeeklyLotBonusRow {
pub fn WeeklyLotBonusParam(&self) -> &ColumnData {
&self.columns[0]
}
}
