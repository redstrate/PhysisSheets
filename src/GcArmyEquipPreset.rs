#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyEquipPreset {
exd: EXD,
exh: EXH,
}
impl GcArmyEquipPreset {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyEquipPreset").unwrap();let exd = game_data.read_excel_sheet("GcArmyEquipPreset", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyEquipPresetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyEquipPresetRow { columns: row.columns.clone() }
}
}
pub struct GcArmyEquipPresetRow {
columns: Vec<ColumnData>,
}
impl GcArmyEquipPresetRow {
pub fn MainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn OffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Head(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Body(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Gloves(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Legs(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Feet(&self) -> &ColumnData {
&self.columns[6]
}
}
