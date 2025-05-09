#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AetheryteSystemDefine {
exd: EXD,
exh: EXH,
}
impl AetheryteSystemDefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AetheryteSystemDefine").unwrap();let exd = game_data.read_excel_sheet("AetheryteSystemDefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AetheryteSystemDefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AetheryteSystemDefineRow { columns: row.columns.clone() }
}
}
pub struct AetheryteSystemDefineRow {
columns: Vec<ColumnData>,
}
impl AetheryteSystemDefineRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DefineValue(&self) -> &ColumnData {
&self.columns[1]
}
}
