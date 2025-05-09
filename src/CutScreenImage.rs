#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CutScreenImage {
exd: EXD,
exh: EXH,
}
impl CutScreenImage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CutScreenImage").unwrap();let exd = game_data.read_excel_sheet("CutScreenImage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CutScreenImageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CutScreenImageRow { columns: row.columns.clone() }
}
}
pub struct CutScreenImageRow {
columns: Vec<ColumnData>,
}
impl CutScreenImageRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
}
