#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RacingChocoboNameCategory {
exd: EXD,
exh: EXH,
}
impl RacingChocoboNameCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RacingChocoboNameCategory").unwrap();let exd = game_data.read_excel_sheet("RacingChocoboNameCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RacingChocoboNameCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RacingChocoboNameCategoryRow { columns: row.columns.clone() }
}
}
pub struct RacingChocoboNameCategoryRow {
columns: Vec<ColumnData>,
}
impl RacingChocoboNameCategoryRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[1]
}
}
