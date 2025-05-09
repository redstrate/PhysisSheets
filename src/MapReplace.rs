#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MapReplace {
exd: EXD,
exh: EXH,
}
impl MapReplace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MapReplace").unwrap();let exd = game_data.read_excel_sheet("MapReplace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapReplaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapReplaceRow { columns: row.columns.clone() }
}
}
pub struct MapReplaceRow {
columns: Vec<ColumnData>,
}
impl MapReplaceRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[7]
}
pub fn QuestSequence(&self) -> &ColumnData {
&self.columns[8]
}
}
