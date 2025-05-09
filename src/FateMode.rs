#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FateMode {
exd: EXD,
exh: EXH,
}
impl FateMode {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FateMode").unwrap();let exd = game_data.read_excel_sheet("FateMode", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateModeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FateModeRow { columns: row.columns.clone() }
}
}
pub struct FateModeRow {
columns: Vec<ColumnData>,
}
impl FateModeRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MotivationIcon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MotivationMapMarker(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ObjectiveIcon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ObjectiveMapMarker(&self) -> &ColumnData {
&self.columns[4]
}
}
