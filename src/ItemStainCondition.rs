#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemStainCondition {
exd: EXD,
exh: EXH,
}
impl ItemStainCondition {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemStainCondition").unwrap();let exd = game_data.read_excel_sheet("ItemStainCondition", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemStainConditionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemStainConditionRow { columns: row.columns.clone() }
}
}
pub struct ItemStainConditionRow {
columns: Vec<ColumnData>,
}
impl ItemStainConditionRow {
pub fn UnlockQuest(&self) -> &ColumnData {
&self.columns[0]
}
}
