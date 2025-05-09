#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GilShopItem {
exd: EXD,
exh: EXH,
}
impl GilShopItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GilShopItem").unwrap();let exd = game_data.read_excel_sheet("GilShopItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GilShopItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GilShopItemRow { columns: row.columns.clone() }
}
}
pub struct GilShopItemRow {
columns: Vec<ColumnData>,
}
impl GilShopItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn QuestRequired(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AchievementRequired(&self) -> &ColumnData {
&self.columns[2]
}
pub fn StateRequired(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Patch(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn IsHQ(&self) -> &ColumnData {
&self.columns[8]
}
}
