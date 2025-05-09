#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HowToPage {
exd: EXD,
exh: EXH,
}
impl HowToPage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HowToPage").unwrap();let exd = game_data.read_excel_sheet("HowToPage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HowToPageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HowToPageRow { columns: row.columns.clone() }
}
}
pub struct HowToPageRow {
columns: Vec<ColumnData>,
}
impl HowToPageRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IconType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TextType(&self) -> &ColumnData {
&self.columns[4]
}
}
