#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaMakeCustomize {
exd: EXD,
exh: EXH,
}
impl CharaMakeCustomize {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaMakeCustomize").unwrap();let exd = game_data.read_excel_sheet("CharaMakeCustomize", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaMakeCustomizeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaMakeCustomizeRow { columns: row.columns.clone() }
}
}
pub struct CharaMakeCustomizeRow {
columns: Vec<ColumnData>,
}
impl CharaMakeCustomizeRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Hint(&self) -> &ColumnData {
&self.columns[1]
}
pub fn HintItem(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FeatureID(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn IsPurchasable(&self) -> &ColumnData {
&self.columns[6]
}
}
