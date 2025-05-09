#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CompanyCraftSequence {
exd: EXD,
exh: EXH,
}
impl CompanyCraftSequence {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CompanyCraftSequence").unwrap();let exd = game_data.read_excel_sheet("CompanyCraftSequence", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CompanyCraftSequenceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CompanyCraftSequenceRow { columns: row.columns.clone() }
}
}
pub struct CompanyCraftSequenceRow {
columns: Vec<ColumnData>,
}
impl CompanyCraftSequenceRow {
pub fn Order(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ResultItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CompanyCraftDraftCategory(&self) -> &ColumnData {
&self.columns[3]
}
pub fn CompanyCraftType(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CompanyCraftDraft(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CompanyCraftPart(&self) -> &ColumnData {
&self.columns[6]
}
}
