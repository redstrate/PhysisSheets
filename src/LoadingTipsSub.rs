#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LoadingTipsSub {
exd: EXD,
exh: EXH,
}
impl LoadingTipsSub {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LoadingTipsSub").unwrap();let exd = game_data.read_excel_sheet("LoadingTipsSub", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LoadingTipsSubRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LoadingTipsSubRow { columns: row.columns.clone() }
}
}
pub struct LoadingTipsSubRow {
columns: Vec<ColumnData>,
}
impl LoadingTipsSubRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
