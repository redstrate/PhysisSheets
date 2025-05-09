#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestClassJobSupply {
exd: EXD,
exh: EXH,
}
impl QuestClassJobSupply {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestClassJobSupply").unwrap();let exd = game_data.read_excel_sheet("QuestClassJobSupply", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestClassJobSupplyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestClassJobSupplyRow { columns: row.columns.clone() }
}
}
pub struct QuestClassJobSupplyRow {
columns: Vec<ColumnData>,
}
impl QuestClassJobSupplyRow {
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ENpcResident(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[8]
}
pub fn AmountRequired(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ItemHQ(&self) -> &ColumnData {
&self.columns[10]
}
}
