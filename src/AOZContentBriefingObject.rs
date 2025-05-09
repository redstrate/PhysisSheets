#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AOZContentBriefingObject {
exd: EXD,
exh: EXH,
}
impl AOZContentBriefingObject {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZContentBriefingObject").unwrap();let exd = game_data.read_excel_sheet("AOZContentBriefingObject", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZContentBriefingObjectRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AOZContentBriefingObjectRow { columns: row.columns.clone() }
}
}
pub struct AOZContentBriefingObjectRow {
columns: Vec<ColumnData>,
}
impl AOZContentBriefingObjectRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
