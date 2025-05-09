#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Pet {
exd: EXD,
exh: EXH,
}
impl Pet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Pet").unwrap();let exd = game_data.read_excel_sheet("Pet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PetRow { columns: row.columns.clone() }
}
}
pub struct PetRow {
columns: Vec<ColumnData>,
}
impl PetRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Abilities(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AutoAction(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SmallScalePercentage(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MediumScalePercentage(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LargeScalePercentage(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[13]
}
pub fn NonCombatSummon(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[15]
}
}
