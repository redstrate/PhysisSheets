#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionCostType {
exd: EXD,
exh: EXH,
}
impl ActionCostType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionCostType").unwrap();let exd = game_data.read_excel_sheet("ActionCostType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionCostTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionCostTypeRow { columns: row.columns.clone() }
}
}
pub struct ActionCostTypeRow {
columns: Vec<ColumnData>,
}
impl ActionCostTypeRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
