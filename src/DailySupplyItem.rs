#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DailySupplyItem {
exd: EXD,
exh: EXH,
}
impl DailySupplyItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DailySupplyItem").unwrap();let exd = game_data.read_excel_sheet("DailySupplyItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DailySupplyItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DailySupplyItemRow { columns: row.columns.clone() }
}
}
pub struct DailySupplyItemRow {
columns: Vec<ColumnData>,
}
impl DailySupplyItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quantity(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RecipeLevel(&self) -> &ColumnData {
&self.columns[2]
}
}
