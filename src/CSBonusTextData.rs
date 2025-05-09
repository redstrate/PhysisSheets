#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CSBonusTextData {
exd: EXD,
exh: EXH,
}
impl CSBonusTextData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusTextData").unwrap();let exd = game_data.read_excel_sheet("CSBonusTextData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusTextDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CSBonusTextDataRow { columns: row.columns.clone() }
}
}
pub struct CSBonusTextDataRow {
columns: Vec<ColumnData>,
}
impl CSBonusTextDataRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
