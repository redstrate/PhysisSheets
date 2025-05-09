#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaMagiciteItemType {
exd: EXD,
exh: EXH,
}
impl EurekaMagiciteItemType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaMagiciteItemType").unwrap();let exd = game_data.read_excel_sheet("EurekaMagiciteItemType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaMagiciteItemTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaMagiciteItemTypeRow { columns: row.columns.clone() }
}
}
pub struct EurekaMagiciteItemTypeRow {
columns: Vec<ColumnData>,
}
impl EurekaMagiciteItemTypeRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
}
