#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GFateClimbing2Content {
exd: EXD,
exh: EXH,
}
impl GFateClimbing2Content {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GFateClimbing2Content").unwrap();let exd = game_data.read_excel_sheet("GFateClimbing2Content", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GFateClimbing2ContentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GFateClimbing2ContentRow { columns: row.columns.clone() }
}
}
pub struct GFateClimbing2ContentRow {
columns: Vec<ColumnData>,
}
impl GFateClimbing2ContentRow {
pub fn PublicContentTextData(&self) -> &ColumnData {
&self.columns[0]
}
}
