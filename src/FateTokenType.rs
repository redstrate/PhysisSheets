#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FateTokenType {
exd: EXD,
exh: EXH,
}
impl FateTokenType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FateTokenType").unwrap();let exd = game_data.read_excel_sheet("FateTokenType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateTokenTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FateTokenTypeRow { columns: row.columns.clone() }
}
}
pub struct FateTokenTypeRow {
columns: Vec<ColumnData>,
}
impl FateTokenTypeRow {
pub fn Currency(&self) -> &ColumnData {
&self.columns[0]
}
}
