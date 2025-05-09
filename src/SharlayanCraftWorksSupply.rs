#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SharlayanCraftWorksSupply {
exd: EXD,
exh: EXH,
}
impl SharlayanCraftWorksSupply {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SharlayanCraftWorksSupply").unwrap();let exd = game_data.read_excel_sheet("SharlayanCraftWorksSupply", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SharlayanCraftWorksSupplyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SharlayanCraftWorksSupplyRow { columns: row.columns.clone() }
}
}
pub struct SharlayanCraftWorksSupplyRow {
columns: Vec<ColumnData>,
}
impl SharlayanCraftWorksSupplyRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}
