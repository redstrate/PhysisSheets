#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestLinkMarkerIcon {
exd: EXD,
exh: EXH,
}
impl QuestLinkMarkerIcon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestLinkMarkerIcon").unwrap();let exd = game_data.read_excel_sheet("QuestLinkMarkerIcon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestLinkMarkerIconRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestLinkMarkerIconRow { columns: row.columns.clone() }
}
}
pub struct QuestLinkMarkerIconRow {
columns: Vec<ColumnData>,
}
impl QuestLinkMarkerIconRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
}
