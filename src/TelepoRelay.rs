#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TelepoRelay {
exd: EXD,
exh: EXH,
}
impl TelepoRelay {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TelepoRelay").unwrap();let exd = game_data.read_excel_sheet("TelepoRelay", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TelepoRelayRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TelepoRelayRow { columns: row.columns.clone() }
}
}
pub struct TelepoRelayRow {
columns: Vec<ColumnData>,
}
impl TelepoRelayRow {
pub fn Relays(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[1]
}
}
