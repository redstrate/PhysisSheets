#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DawnContentParticipable {
exd: EXD,
exh: EXH,
}
impl DawnContentParticipable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DawnContentParticipable").unwrap();let exd = game_data.read_excel_sheet("DawnContentParticipable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DawnContentParticipableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DawnContentParticipableRow { columns: row.columns.clone() }
}
}
pub struct DawnContentParticipableRow {
columns: Vec<ColumnData>,
}
impl DawnContentParticipableRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
