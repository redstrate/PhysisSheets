#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCAuthority {
exd: EXD,
exh: EXH,
}
impl FCAuthority {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCAuthority").unwrap();let exd = game_data.read_excel_sheet("FCAuthority", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCAuthorityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCAuthorityRow { columns: row.columns.clone() }
}
}
pub struct FCAuthorityRow {
columns: Vec<ColumnData>,
}
impl FCAuthorityRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn FCAuthorityCategory(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
}
