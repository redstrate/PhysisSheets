#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDInfoBoardArticleType {
exd: EXD,
exh: EXH,
}
impl HWDInfoBoardArticleType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDInfoBoardArticleType").unwrap();let exd = game_data.read_excel_sheet("HWDInfoBoardArticleType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDInfoBoardArticleTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDInfoBoardArticleTypeRow { columns: row.columns.clone() }
}
}
pub struct HWDInfoBoardArticleTypeRow {
columns: Vec<ColumnData>,
}
impl HWDInfoBoardArticleTypeRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
}
