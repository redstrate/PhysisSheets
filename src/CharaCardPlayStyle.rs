#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaCardPlayStyle {
exd: EXD,
exh: EXH,
}
impl CharaCardPlayStyle {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaCardPlayStyle").unwrap();let exd = game_data.read_excel_sheet("CharaCardPlayStyle", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaCardPlayStyleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaCardPlayStyleRow { columns: row.columns.clone() }
}
}
pub struct CharaCardPlayStyleRow {
columns: Vec<ColumnData>,
}
impl CharaCardPlayStyleRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[2]
}
}
