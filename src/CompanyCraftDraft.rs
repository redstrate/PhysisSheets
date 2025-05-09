#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftDraft {
exd: EXD,
exh: EXH,
}
impl CompanyCraftDraft {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftDraft").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftDraft", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftDraftRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftDraftRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftDraftRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftDraftRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredItem(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CompanyCraftDraftCategory(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RequiredItemCount(&self) -> &ColumnData {
&self.columns[4]
}
}
