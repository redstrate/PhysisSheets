#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GimmickBill {
exd: EXD,
exh: EXH,
}
impl GimmickBill {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GimmickBill").unwrap();let exd = game_data.read_excel_sheet("GimmickBill", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GimmickBillRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GimmickBillRow { columns: row.columns.clone() }
}
}
pub struct GimmickBillRow {
columns: Vec<ColumnData>,
}
impl GimmickBillRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
