#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct InclusionShopWelcomText {
exd: EXD,
exh: EXH,
}
impl InclusionShopWelcomText {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("InclusionShopWelcomText").unwrap();let exd = game_data.read_excel_sheet("InclusionShopWelcomText", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> InclusionShopWelcomTextRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
InclusionShopWelcomTextRow { columns: row.columns.clone() }
}
}
pub struct InclusionShopWelcomTextRow {
columns: Vec<ColumnData>,
}
impl InclusionShopWelcomTextRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
