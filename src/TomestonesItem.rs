#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TomestonesItem {
exd: EXD,
exh: EXH,
}
impl TomestonesItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TomestonesItem").unwrap();let exd = game_data.read_excel_sheet("TomestonesItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TomestonesItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TomestonesItemRow { columns: row.columns.clone() }
}
}
pub struct TomestonesItemRow {
columns: Vec<ColumnData>,
}
impl TomestonesItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Tomestones(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CurrencyInventorySlot(&self) -> &ColumnData {
&self.columns[2]
}
}
