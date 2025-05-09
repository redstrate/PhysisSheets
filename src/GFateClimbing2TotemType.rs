#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GFateClimbing2TotemType {
exd: EXD,
exh: EXH,
}
impl GFateClimbing2TotemType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GFateClimbing2TotemType").unwrap();let exd = game_data.read_excel_sheet("GFateClimbing2TotemType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GFateClimbing2TotemTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GFateClimbing2TotemTypeRow { columns: row.columns.clone() }
}
}
pub struct GFateClimbing2TotemTypeRow {
columns: Vec<ColumnData>,
}
impl GFateClimbing2TotemTypeRow {
pub fn PublicContentTextData(&self) -> &ColumnData {
&self.columns[0]
}
}
