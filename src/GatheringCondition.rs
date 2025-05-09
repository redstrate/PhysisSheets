#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringCondition {
exd: EXD,
exh: EXH,
}
impl GatheringCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringCondition").unwrap();let exd = game_data.read_excel_sheet("GatheringCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringConditionRow { columns: row.columns.clone() }
}
}
pub struct GatheringConditionRow {
columns: Vec<ColumnData>,
}
impl GatheringConditionRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
