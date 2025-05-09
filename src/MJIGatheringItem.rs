#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIGatheringItem {
exd: EXD,
exh: EXH,
}
impl MJIGatheringItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIGatheringItem").unwrap();let exd = game_data.read_excel_sheet("MJIGatheringItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIGatheringItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIGatheringItemRow { columns: row.columns.clone() }
}
}
pub struct MJIGatheringItemRow {
columns: Vec<ColumnData>,
}
impl MJIGatheringItemRow {
pub fn Radius(&self) -> &ColumnData {
&self.columns[0]
}
pub fn X(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Sort(&self) -> &ColumnData {
&self.columns[6]
}
}
