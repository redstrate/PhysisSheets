#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AOZScore {
exd: EXD,
exh: EXH,
}
impl AOZScore {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZScore").unwrap();let exd = game_data.read_excel_sheet("AOZScore", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZScoreRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AOZScoreRow { columns: row.columns.clone() }
}
}
pub struct AOZScoreRow {
columns: Vec<ColumnData>,
}
impl AOZScoreRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Score(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IsVisible(&self) -> &ColumnData {
&self.columns[3]
}
}
