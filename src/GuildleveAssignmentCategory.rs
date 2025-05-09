#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GuildleveAssignmentCategory {
exd: EXD,
exh: EXH,
}
impl GuildleveAssignmentCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GuildleveAssignmentCategory").unwrap();let exd = game_data.read_excel_sheet("GuildleveAssignmentCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GuildleveAssignmentCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GuildleveAssignmentCategoryRow { columns: row.columns.clone() }
}
}
pub struct GuildleveAssignmentCategoryRow {
columns: Vec<ColumnData>,
}
impl GuildleveAssignmentCategoryRow {
pub fn Category(&self) -> &ColumnData {
&self.columns[0]
}
}
