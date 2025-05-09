#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ClassJobActionUI {
exd: EXD,
exh: EXH,
}
impl ClassJobActionUI {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ClassJobActionUI").unwrap();let exd = game_data.read_excel_sheet("ClassJobActionUI", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ClassJobActionUIRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ClassJobActionUIRow { columns: row.columns.clone() }
}
}
pub struct ClassJobActionUIRow {
columns: Vec<ColumnData>,
}
impl ClassJobActionUIRow {
pub fn UpgradeAction(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BaseAction(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ComboTreeLayout(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GroupedCell(&self) -> &ColumnData {
&self.columns[6]
}
}
