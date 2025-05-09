#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RetainerFortuneRewardRange {
exd: EXD,
exh: EXH,
}
impl RetainerFortuneRewardRange {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RetainerFortuneRewardRange").unwrap();let exd = game_data.read_excel_sheet("RetainerFortuneRewardRange", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RetainerFortuneRewardRangeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RetainerFortuneRewardRangeRow { columns: row.columns.clone() }
}
}
pub struct RetainerFortuneRewardRangeRow {
columns: Vec<ColumnData>,
}
impl RetainerFortuneRewardRangeRow {
pub fn PercentOfLevel(&self) -> &ColumnData {
&self.columns[0]
}
}
