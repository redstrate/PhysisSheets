#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingFurniture {
exd: EXD,
exh: EXH,
}
impl HousingFurniture {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingFurniture").unwrap();let exd = game_data.read_excel_sheet("HousingFurniture", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingFurnitureRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingFurnitureRow { columns: row.columns.clone() }
}
}
pub struct HousingFurnitureRow {
columns: Vec<ColumnData>,
}
impl HousingFurnitureRow {
pub fn UsageParameter(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CustomTalk(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ModelKey(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HousingItemCategory(&self) -> &ColumnData {
&self.columns[4]
}
pub fn UsageType(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn AquariumTier(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[10]
}
pub fn DestroyOnRemoval(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[14]
}
}
