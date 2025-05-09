#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WorldDCGroupType {
exd: EXD,
exh: EXH,
}
impl WorldDCGroupType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WorldDCGroupType").unwrap();let exd = game_data.read_excel_sheet("WorldDCGroupType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WorldDCGroupTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WorldDCGroupTypeRow { columns: row.columns.clone() }
}
}
pub struct WorldDCGroupTypeRow {
columns: Vec<ColumnData>,
}
impl WorldDCGroupTypeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PvPRegion(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NeolobbyId(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Region(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IsCloud(&self) -> &ColumnData {
&self.columns[4]
}
}
