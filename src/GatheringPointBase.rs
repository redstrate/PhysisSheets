#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringPointBase {
exd: EXD,
exh: EXH,
}
impl GatheringPointBase {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringPointBase").unwrap();let exd = game_data.read_excel_sheet("GatheringPointBase", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringPointBaseRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringPointBaseRow { columns: row.columns.clone() }
}
}
pub struct GatheringPointBaseRow {
columns: Vec<ColumnData>,
}
impl GatheringPointBaseRow {
pub fn GatheringType(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn GatheringLevel(&self) -> &ColumnData {
&self.columns[2]
}
}
