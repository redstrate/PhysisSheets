#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MinionRules {
exd: EXD,
exh: EXH,
}
impl MinionRules {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MinionRules").unwrap();let exd = game_data.read_excel_sheet("MinionRules", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MinionRulesRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MinionRulesRow { columns: row.columns.clone() }
}
}
pub struct MinionRulesRow {
columns: Vec<ColumnData>,
}
impl MinionRulesRow {
pub fn Rule(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
}
