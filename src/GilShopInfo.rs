#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GilShopInfo {
exd: EXD,
exh: EXH,
}
impl GilShopInfo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GilShopInfo").unwrap();let exd = game_data.read_excel_sheet("GilShopInfo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GilShopInfoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GilShopInfoRow { columns: row.columns.clone() }
}
}
pub struct GilShopInfoRow {
columns: Vec<ColumnData>,
}
impl GilShopInfoRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
