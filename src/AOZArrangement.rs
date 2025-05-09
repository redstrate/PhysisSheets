#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AOZArrangement {
exd: EXD,
exh: EXH,
}
impl AOZArrangement {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZArrangement").unwrap();let exd = game_data.read_excel_sheet("AOZArrangement", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZArrangementRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AOZArrangementRow { columns: row.columns.clone() }
}
}
pub struct AOZArrangementRow {
columns: Vec<ColumnData>,
}
impl AOZArrangementRow {
pub fn AOZContentBriefingBNpc(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Position(&self) -> &ColumnData {
&self.columns[1]
}
}
