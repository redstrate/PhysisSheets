#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CollectablesShopRefine {
exd: EXD,
exh: EXH,
}
impl CollectablesShopRefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CollectablesShopRefine").unwrap();let exd = game_data.read_excel_sheet("CollectablesShopRefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CollectablesShopRefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CollectablesShopRefineRow { columns: row.columns.clone() }
}
}
pub struct CollectablesShopRefineRow {
columns: Vec<ColumnData>,
}
impl CollectablesShopRefineRow {
pub fn LowCollectability(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MidCollectability(&self) -> &ColumnData {
&self.columns[1]
}
pub fn HighCollectability(&self) -> &ColumnData {
&self.columns[2]
}
}
