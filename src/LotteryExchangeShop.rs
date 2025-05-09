#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LotteryExchangeShop {
exd: EXD,
exh: EXH,
}
impl LotteryExchangeShop {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LotteryExchangeShop").unwrap();let exd = game_data.read_excel_sheet("LotteryExchangeShop", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LotteryExchangeShopRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LotteryExchangeShopRow { columns: row.columns.clone() }
}
}
pub struct LotteryExchangeShopRow {
columns: Vec<ColumnData>,
}
impl LotteryExchangeShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn LotteryExchangeParams(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Script(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LogMessage(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
}
