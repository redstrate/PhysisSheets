#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SecretRecipeBook {
exd: EXD,
exh: EXH,
}
impl SecretRecipeBook {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SecretRecipeBook").unwrap();let exd = game_data.read_excel_sheet("SecretRecipeBook", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SecretRecipeBookRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SecretRecipeBookRow { columns: row.columns.clone() }
}
}
pub struct SecretRecipeBookRow {
columns: Vec<ColumnData>,
}
impl SecretRecipeBookRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
}
