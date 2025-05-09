#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SatisfactionArbitration {
exd: EXD,
exh: EXH,
}
impl SatisfactionArbitration {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SatisfactionArbitration").unwrap();let exd = game_data.read_excel_sheet("SatisfactionArbitration", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SatisfactionArbitrationRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SatisfactionArbitrationRow { columns: row.columns.clone() }
}
}
pub struct SatisfactionArbitrationRow {
columns: Vec<ColumnData>,
}
impl SatisfactionArbitrationRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SatisfactionLevel(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SatisfactionNpc(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
