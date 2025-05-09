#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BgcArmyActionTransient {
exd: EXD,
exh: EXH,
}
impl BgcArmyActionTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BgcArmyActionTransient").unwrap();let exd = game_data.read_excel_sheet("BgcArmyActionTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BgcArmyActionTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BgcArmyActionTransientRow { columns: row.columns.clone() }
}
}
pub struct BgcArmyActionTransientRow {
columns: Vec<ColumnData>,
}
impl BgcArmyActionTransientRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
