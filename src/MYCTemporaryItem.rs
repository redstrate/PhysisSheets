#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MYCTemporaryItem {
exd: EXD,
exh: EXH,
}
impl MYCTemporaryItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MYCTemporaryItem").unwrap();let exd = game_data.read_excel_sheet("MYCTemporaryItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MYCTemporaryItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MYCTemporaryItemRow { columns: row.columns.clone() }
}
}
pub struct MYCTemporaryItemRow {
columns: Vec<ColumnData>,
}
impl MYCTemporaryItemRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Max(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Weight(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[5]
}
}
