#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ExportedGatheringPoint {
exd: EXD,
exh: EXH,
}
impl ExportedGatheringPoint {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ExportedGatheringPoint").unwrap();let exd = game_data.read_excel_sheet("ExportedGatheringPoint", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ExportedGatheringPointRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ExportedGatheringPointRow { columns: row.columns.clone() }
}
}
pub struct ExportedGatheringPointRow {
columns: Vec<ColumnData>,
}
impl ExportedGatheringPointRow {
pub fn X(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Radius(&self) -> &ColumnData {
&self.columns[2]
}
pub fn GatheringType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GatheringPointType(&self) -> &ColumnData {
&self.columns[4]
}
}
