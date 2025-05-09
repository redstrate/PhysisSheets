#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CraftLeve {
exd: EXD,
exh: EXH,
}
impl CraftLeve {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CraftLeve").unwrap();let exd = game_data.read_excel_sheet("CraftLeve", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CraftLeveRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CraftLeveRow { columns: row.columns.clone() }
}
}
pub struct CraftLeveRow {
columns: Vec<ColumnData>,
}
impl CraftLeveRow {
pub fn Leve(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CraftLeveTalk(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ItemCount(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Repeats(&self) -> &ColumnData {
&self.columns[4]
}
}
