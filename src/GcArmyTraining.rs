#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyTraining {
exd: EXD,
exh: EXH,
}
impl GcArmyTraining {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyTraining").unwrap();let exd = game_data.read_excel_sheet("GcArmyTraining", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyTrainingRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyTrainingRow { columns: row.columns.clone() }
}
}
pub struct GcArmyTrainingRow {
columns: Vec<ColumnData>,
}
impl GcArmyTrainingRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Experience(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PhysicalBonus(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MentalBonus(&self) -> &ColumnData {
&self.columns[4]
}
pub fn TacticalBonus(&self) -> &ColumnData {
&self.columns[5]
}
}
