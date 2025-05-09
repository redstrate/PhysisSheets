#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RecipeLevelTable {
exd: EXD,
exh: EXH,
}
impl RecipeLevelTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RecipeLevelTable").unwrap();let exd = game_data.read_excel_sheet("RecipeLevelTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RecipeLevelTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RecipeLevelTableRow { columns: row.columns.clone() }
}
}
pub struct RecipeLevelTableRow {
columns: Vec<ColumnData>,
}
impl RecipeLevelTableRow {
pub fn Quality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SuggestedCraftsmanship(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Difficulty(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Durability(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ConditionsFlag(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ProgressDivider(&self) -> &ColumnData {
&self.columns[7]
}
pub fn QualityDivider(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ProgressModifier(&self) -> &ColumnData {
&self.columns[9]
}
pub fn QualityModifier(&self) -> &ColumnData {
&self.columns[10]
}
}
