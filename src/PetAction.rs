#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PetAction {
exd: EXD,
exh: EXH,
}
impl PetAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PetAction").unwrap();let exd = game_data.read_excel_sheet("PetAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PetActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PetActionRow { columns: row.columns.clone() }
}
}
pub struct PetActionRow {
columns: Vec<ColumnData>,
}
impl PetActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Pet(&self) -> &ColumnData {
&self.columns[4]
}
pub fn MasterOrder(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DisableOrder(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
}
