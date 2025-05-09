#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct VVDData {
exd: EXD,
exh: EXH,
}
impl VVDData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("VVDData").unwrap();let exd = game_data.read_excel_sheet("VVDData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> VVDDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
VVDDataRow { columns: row.columns.clone() }
}
}
pub struct VVDDataRow {
columns: Vec<ColumnData>,
}
impl VVDDataRow {
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CurrencyItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ContentExAction(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UnlockQuest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Series(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[5]
}
}
