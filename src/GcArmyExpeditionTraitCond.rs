#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyExpeditionTraitCond {
exd: EXD,
exh: EXH,
}
impl GcArmyExpeditionTraitCond {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyExpeditionTraitCond").unwrap();let exd = game_data.read_excel_sheet("GcArmyExpeditionTraitCond", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyExpeditionTraitCondRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyExpeditionTraitCondRow { columns: row.columns.clone() }
}
}
pub struct GcArmyExpeditionTraitCondRow {
columns: Vec<ColumnData>,
}
impl GcArmyExpeditionTraitCondRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
