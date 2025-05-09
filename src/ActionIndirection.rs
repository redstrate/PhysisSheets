#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ActionIndirection {
exd: EXD,
exh: EXH,
}
impl ActionIndirection {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionIndirection").unwrap();let exd = game_data.read_excel_sheet("ActionIndirection", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionIndirectionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ActionIndirectionRow { columns: row.columns.clone() }
}
}
pub struct ActionIndirectionRow {
columns: Vec<ColumnData>,
}
impl ActionIndirectionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PreviousComboAction(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[2]
}
}
