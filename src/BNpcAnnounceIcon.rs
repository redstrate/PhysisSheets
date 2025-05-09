#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BNpcAnnounceIcon {
exd: EXD,
exh: EXH,
}
impl BNpcAnnounceIcon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BNpcAnnounceIcon").unwrap();let exd = game_data.read_excel_sheet("BNpcAnnounceIcon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BNpcAnnounceIconRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BNpcAnnounceIconRow { columns: row.columns.clone() }
}
}
pub struct BNpcAnnounceIconRow {
columns: Vec<ColumnData>,
}
impl BNpcAnnounceIconRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
}
