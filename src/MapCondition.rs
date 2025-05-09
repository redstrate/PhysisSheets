#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MapCondition {
exd: EXD,
exh: EXH,
}
impl MapCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MapCondition").unwrap();let exd = game_data.read_excel_sheet("MapCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapConditionRow { columns: row.columns.clone() }
}
}
pub struct MapConditionRow {
columns: Vec<ColumnData>,
}
impl MapConditionRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn InstanceContent(&self) -> &ColumnData {
&self.columns[1]
}
pub fn QuestSequence(&self) -> &ColumnData {
&self.columns[2]
}
}
