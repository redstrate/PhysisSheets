#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeddingBGM {
exd: EXD,
exh: EXH,
}
impl WeddingBGM {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeddingBGM").unwrap();let exd = game_data.read_excel_sheet("WeddingBGM", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeddingBGMRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeddingBGMRow { columns: row.columns.clone() }
}
}
pub struct WeddingBGMRow {
columns: Vec<ColumnData>,
}
impl WeddingBGMRow {
pub fn SongName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Song(&self) -> &ColumnData {
&self.columns[1]
}
}
