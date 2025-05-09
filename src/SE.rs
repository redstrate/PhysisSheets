#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SE {
exd: EXD,
exh: EXH,
}
impl SE {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SE").unwrap();let exd = game_data.read_excel_sheet("SE", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SERow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SERow { columns: row.columns.clone() }
}
}
pub struct SERow {
columns: Vec<ColumnData>,
}
impl SERow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
