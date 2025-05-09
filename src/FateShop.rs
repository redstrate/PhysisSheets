#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FateShop {
exd: EXD,
exh: EXH,
}
impl FateShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FateShop").unwrap();let exd = game_data.read_excel_sheet("FateShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FateShopRow { columns: row.columns.clone() }
}
}
pub struct FateShopRow {
columns: Vec<ColumnData>,
}
impl FateShopRow {
pub fn SpecialShop(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DefaultTalk(&self) -> &ColumnData {
&self.columns[1]
}
}
