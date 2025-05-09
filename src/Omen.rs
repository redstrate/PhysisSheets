#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Omen {
exd: EXD,
exh: EXH,
}
impl Omen {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Omen").unwrap();let exd = game_data.read_excel_sheet("Omen", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OmenRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
OmenRow { columns: row.columns.clone() }
}
}
pub struct OmenRow {
columns: Vec<ColumnData>,
}
impl OmenRow {
pub fn Path(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PathAlly(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RestrictYScale(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LargeScale(&self) -> &ColumnData {
&self.columns[5]
}
}
