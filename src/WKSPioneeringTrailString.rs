#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WKSPioneeringTrailString {
exd: EXD,
exh: EXH,
}
impl WKSPioneeringTrailString {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WKSPioneeringTrailString").unwrap();let exd = game_data.read_excel_sheet("WKSPioneeringTrailString", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WKSPioneeringTrailStringRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WKSPioneeringTrailStringRow { columns: row.columns.clone() }
}
}
pub struct WKSPioneeringTrailStringRow {
columns: Vec<ColumnData>,
}
impl WKSPioneeringTrailStringRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
}
