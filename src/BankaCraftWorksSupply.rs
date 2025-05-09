#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BankaCraftWorksSupply {
exd: EXD,
exh: EXH,
}
impl BankaCraftWorksSupply {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BankaCraftWorksSupply").unwrap();let exd = game_data.read_excel_sheet("BankaCraftWorksSupply", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BankaCraftWorksSupplyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BankaCraftWorksSupplyRow { columns: row.columns.clone() }
}
}
pub struct BankaCraftWorksSupplyRow {
columns: Vec<ColumnData>,
}
impl BankaCraftWorksSupplyRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
}
