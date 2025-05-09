#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GathererCrafterLvAdjustTable {
exd: EXD,
exh: EXH,
}
impl GathererCrafterLvAdjustTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GathererCrafterLvAdjustTable").unwrap();let exd = game_data.read_excel_sheet("GathererCrafterLvAdjustTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GathererCrafterLvAdjustTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GathererCrafterLvAdjustTableRow { columns: row.columns.clone() }
}
}
pub struct GathererCrafterLvAdjustTableRow {
columns: Vec<ColumnData>,
}
impl GathererCrafterLvAdjustTableRow {
pub fn RecipeLevel(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CrafterLevel(&self) -> &ColumnData {
&self.columns[2]
}
pub fn GathererLevel(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FisherLevel(&self) -> &ColumnData {
&self.columns[4]
}
}
