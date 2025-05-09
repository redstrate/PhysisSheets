#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GuidePage {
exd: EXD,
exh: EXH,
}
impl GuidePage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GuidePage").unwrap();let exd = game_data.read_excel_sheet("GuidePage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GuidePageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GuidePageRow { columns: row.columns.clone() }
}
}
pub struct GuidePageRow {
columns: Vec<ColumnData>,
}
impl GuidePageRow {
pub fn Output(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[2]
}
}
