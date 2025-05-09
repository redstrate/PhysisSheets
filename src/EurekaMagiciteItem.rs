#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaMagiciteItem {
exd: EXD,
exh: EXH,
}
impl EurekaMagiciteItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaMagiciteItem").unwrap();let exd = game_data.read_excel_sheet("EurekaMagiciteItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaMagiciteItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaMagiciteItemRow { columns: row.columns.clone() }
}
}
pub struct EurekaMagiciteItemRow {
columns: Vec<ColumnData>,
}
impl EurekaMagiciteItemRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EurekaMagiciteItemType(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[2]
}
}
