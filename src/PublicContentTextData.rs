#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PublicContentTextData {
exd: EXD,
exh: EXH,
}
impl PublicContentTextData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PublicContentTextData").unwrap();let exd = game_data.read_excel_sheet("PublicContentTextData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PublicContentTextDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PublicContentTextDataRow { columns: row.columns.clone() }
}
}
pub struct PublicContentTextDataRow {
columns: Vec<ColumnData>,
}
impl PublicContentTextDataRow {
pub fn TextData(&self) -> &ColumnData {
&self.columns[0]
}
}
