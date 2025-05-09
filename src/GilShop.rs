#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GilShop {
exd: EXD,
exh: EXH,
}
impl GilShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GilShop").unwrap();let exd = game_data.read_excel_sheet("GilShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GilShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GilShopRow { columns: row.columns.clone() }
}
}
pub struct GilShopRow {
columns: Vec<ColumnData>,
}
impl GilShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[2]
}
pub fn AcceptTalk(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FailTalk(&self) -> &ColumnData {
&self.columns[4]
}
pub fn FestivalId(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FestivalPhase(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[7]
}
}
