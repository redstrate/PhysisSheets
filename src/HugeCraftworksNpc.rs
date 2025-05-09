#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HugeCraftworksNpc {
exd: EXD,
exh: EXH,
}
impl HugeCraftworksNpc {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HugeCraftworksNpc").unwrap();let exd = game_data.read_excel_sheet("HugeCraftworksNpc", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HugeCraftworksNpcRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HugeCraftworksNpcRow { columns: row.columns.clone() }
}
}
pub struct HugeCraftworksNpcRow {
columns: Vec<ColumnData>,
}
impl HugeCraftworksNpcRow {
pub fn HugeCraftworksTurnInParam(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HugeCraftworksRewardParam(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Transient(&self) -> &ColumnData {
&self.columns[2]
}
pub fn EventNpc(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[4]
}
}
