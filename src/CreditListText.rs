#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CreditListText {
exd: EXD,
exh: EXH,
}
impl CreditListText {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CreditListText").unwrap();let exd = game_data.read_excel_sheet("CreditListText", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CreditListTextRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CreditListTextRow { columns: row.columns.clone() }
}
}
pub struct CreditListTextRow {
columns: Vec<ColumnData>,
}
impl CreditListTextRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
}
