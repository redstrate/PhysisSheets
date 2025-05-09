#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIStockyardManagementArea {
exd: EXD,
exh: EXH,
}
impl MJIStockyardManagementArea {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIStockyardManagementArea").unwrap();let exd = game_data.read_excel_sheet("MJIStockyardManagementArea", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIStockyardManagementAreaRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIStockyardManagementAreaRow { columns: row.columns.clone() }
}
}
pub struct MJIStockyardManagementAreaRow {
columns: Vec<ColumnData>,
}
impl MJIStockyardManagementAreaRow {
pub fn Area(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RareMaterial(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
}
