#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AnimaWeapon5TradeItem {
exd: EXD,
exh: EXH,
}
impl AnimaWeapon5TradeItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AnimaWeapon5TradeItem").unwrap();let exd = game_data.read_excel_sheet("AnimaWeapon5TradeItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AnimaWeapon5TradeItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AnimaWeapon5TradeItemRow { columns: row.columns.clone() }
}
}
pub struct AnimaWeapon5TradeItemRow {
columns: Vec<ColumnData>,
}
impl AnimaWeapon5TradeItemRow {
pub fn CrystalSand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ReceiveQuantity(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Quantity(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[5]
}
pub fn IsHQ(&self) -> &ColumnData {
&self.columns[6]
}
}
