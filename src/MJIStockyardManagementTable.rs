#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIStockyardManagementTable {
exd: EXD,
exh: EXH,
}
impl MJIStockyardManagementTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIStockyardManagementTable").unwrap();let exd = game_data.read_excel_sheet("MJIStockyardManagementTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIStockyardManagementTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIStockyardManagementTableRow { columns: row.columns.clone() }
}
}
pub struct MJIStockyardManagementTableRow {
columns: Vec<ColumnData>,
}
impl MJIStockyardManagementTableRow {
pub fn Material(&self) -> &ColumnData {
&self.columns[0]
}
}
