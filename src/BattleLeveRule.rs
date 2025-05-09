#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BattleLeveRule {
exd: EXD,
exh: EXH,
}
impl BattleLeveRule {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BattleLeveRule").unwrap();let exd = game_data.read_excel_sheet("BattleLeveRule", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BattleLeveRuleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BattleLeveRuleRow { columns: row.columns.clone() }
}
}
pub struct BattleLeveRuleRow {
columns: Vec<ColumnData>,
}
impl BattleLeveRuleRow {
pub fn Rule(&self) -> &ColumnData {
&self.columns[0]
}
}
