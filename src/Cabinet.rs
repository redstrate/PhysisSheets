#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Cabinet {
exd: EXD,
exh: EXH,
}
impl Cabinet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Cabinet").unwrap();let exd = game_data.read_excel_sheet("Cabinet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CabinetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CabinetRow { columns: row.columns.clone() }
}
}
pub struct CabinetRow {
columns: Vec<ColumnData>,
}
impl CabinetRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SubCategory(&self) -> &ColumnData {
&self.columns[3]
}
}
