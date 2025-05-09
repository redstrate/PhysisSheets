#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSFortunePatternUIColorSet {
exd: EXD,
exh: EXH,
}
impl WKSFortunePatternUIColorSet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSFortunePatternUIColorSet").unwrap();let exd = game_data.read_excel_sheet("WKSFortunePatternUIColorSet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSFortunePatternUIColorSetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSFortunePatternUIColorSetRow { columns: row.columns.clone() }
}
}
pub struct WKSFortunePatternUIColorSetRow {
columns: Vec<ColumnData>,
}
impl WKSFortunePatternUIColorSetRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}
