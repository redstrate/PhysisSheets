#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIRecipe {
exd: EXD,
exh: EXH,
}
impl MJIRecipe {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIRecipe").unwrap();let exd = game_data.read_excel_sheet("MJIRecipe", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIRecipeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIRecipeRow { columns: row.columns.clone() }
}
}
pub struct MJIRecipeRow {
columns: Vec<ColumnData>,
}
impl MJIRecipeRow {
pub fn LogMessage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn KeyItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ItemPouch(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Yield(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Material(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Amount(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[6]
}
}
