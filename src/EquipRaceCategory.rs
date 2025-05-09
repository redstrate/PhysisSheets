#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EquipRaceCategory {
exd: EXD,
exh: EXH,
}
impl EquipRaceCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EquipRaceCategory").unwrap();let exd = game_data.read_excel_sheet("EquipRaceCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EquipRaceCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EquipRaceCategoryRow { columns: row.columns.clone() }
}
}
pub struct EquipRaceCategoryRow {
columns: Vec<ColumnData>,
}
impl EquipRaceCategoryRow {
pub fn Hyur(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Elezen(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Lalafell(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Miqote(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Roegadyn(&self) -> &ColumnData {
&self.columns[4]
}
pub fn AuRa(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Hrothgar(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Viera(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Male(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Female(&self) -> &ColumnData {
&self.columns[9]
}
}
