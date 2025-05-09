#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AquariumFish {
exd: EXD,
exh: EXH,
}
impl AquariumFish {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AquariumFish").unwrap();let exd = game_data.read_excel_sheet("AquariumFish", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AquariumFishRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AquariumFishRow { columns: row.columns.clone() }
}
}
pub struct AquariumFishRow {
columns: Vec<ColumnData>,
}
impl AquariumFishRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AquariumWater(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Size(&self) -> &ColumnData {
&self.columns[3]
}
}
