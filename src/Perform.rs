#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Perform {
exd: EXD,
exh: EXH,
}
impl Perform {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Perform").unwrap();let exd = game_data.read_excel_sheet("Perform", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PerformRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PerformRow { columns: row.columns.clone() }
}
}
pub struct PerformRow {
columns: Vec<ColumnData>,
}
impl PerformRow {
pub fn AnimationPlay01(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AnimationPlay02(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Instrument(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ModelKey(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[4]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[6]
}
pub fn AnimationStart(&self) -> &ColumnData {
&self.columns[7]
}
pub fn AnimationEnd(&self) -> &ColumnData {
&self.columns[8]
}
pub fn AnimationIdle(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Transient(&self) -> &ColumnData {
&self.columns[10]
}
pub fn PerformGroup(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[12]
}
}
