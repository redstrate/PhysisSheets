#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DescriptionSection {
exd: EXD,
exh: EXH,
}
impl DescriptionSection {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DescriptionSection").unwrap();let exd = game_data.read_excel_sheet("DescriptionSection", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DescriptionSectionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DescriptionSectionRow { columns: row.columns.clone() }
}
}
pub struct DescriptionSectionRow {
columns: Vec<ColumnData>,
}
impl DescriptionSectionRow {
pub fn String(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Page(&self) -> &ColumnData {
&self.columns[1]
}
}
