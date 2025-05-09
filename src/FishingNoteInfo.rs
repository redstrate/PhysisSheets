#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FishingNoteInfo {
exd: EXD,
exh: EXH,
}
impl FishingNoteInfo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FishingNoteInfo").unwrap();let exd = game_data.read_excel_sheet("FishingNoteInfo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FishingNoteInfoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FishingNoteInfoRow { columns: row.columns.clone() }
}
}
pub struct FishingNoteInfoRow {
columns: Vec<ColumnData>,
}
impl FishingNoteInfoRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Size(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AquariumWater(&self) -> &ColumnData {
&self.columns[2]
}
pub fn WeatherRestriction(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TimeRestriction(&self) -> &ColumnData {
&self.columns[4]
}
pub fn SpecialConditions(&self) -> &ColumnData {
&self.columns[5]
}
pub fn IsCollectable(&self) -> &ColumnData {
&self.columns[6]
}
}
