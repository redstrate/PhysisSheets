#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringLeveRule {
exd: EXD,
exh: EXH,
}
impl GatheringLeveRule {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringLeveRule").unwrap();let exd = game_data.read_excel_sheet("GatheringLeveRule", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringLeveRuleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringLeveRuleRow { columns: row.columns.clone() }
}
}
pub struct GatheringLeveRuleRow {
columns: Vec<ColumnData>,
}
impl GatheringLeveRuleRow {
pub fn Rule(&self) -> &ColumnData {
&self.columns[0]
}
}
