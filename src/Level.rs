#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Level {
exd: EXD,
exh: EXH,
}
impl Level {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Level").unwrap();let exd = game_data.read_excel_sheet("Level", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LevelRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LevelRow { columns: row.columns.clone() }
}
}
pub struct LevelRow {
columns: Vec<ColumnData>,
}
impl LevelRow {
pub fn X(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Z(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Yaw(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Radius(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Object(&self) -> &ColumnData {
&self.columns[5]
}
pub fn EventId(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Territory(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[9]
}
}
