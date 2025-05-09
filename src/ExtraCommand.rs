#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ExtraCommand {
exd: EXD,
exh: EXH,
}
impl ExtraCommand {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ExtraCommand").unwrap();let exd = game_data.read_excel_sheet("ExtraCommand", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ExtraCommandRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ExtraCommandRow { columns: row.columns.clone() }
}
}
pub struct ExtraCommandRow {
columns: Vec<ColumnData>,
}
impl ExtraCommandRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[3]
}
}
