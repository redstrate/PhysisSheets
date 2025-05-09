#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RetainerTaskNormal {
exd: EXD,
exh: EXH,
}
impl RetainerTaskNormal {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RetainerTaskNormal").unwrap();let exd = game_data.read_excel_sheet("RetainerTaskNormal", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RetainerTaskNormalRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RetainerTaskNormalRow { columns: row.columns.clone() }
}
}
pub struct RetainerTaskNormalRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskNormalRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GatheringLog(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FishingLog(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quantity(&self) -> &ColumnData {
&self.columns[3]
}
}
