#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TradeScreenImage {
exd: EXD,
exh: EXH,
}
impl TradeScreenImage {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TradeScreenImage").unwrap();let exd = game_data.read_excel_sheet("TradeScreenImage", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TradeScreenImageRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TradeScreenImageRow { columns: row.columns.clone() }
}
}
pub struct TradeScreenImageRow {
columns: Vec<ColumnData>,
}
impl TradeScreenImageRow {
pub fn Items(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemIcons(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ItemValues(&self) -> &ColumnData {
&self.columns[2]
}
pub fn BannerType(&self) -> &ColumnData {
&self.columns[3]
}
}
