#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MoveTimeline {
exd: EXD,
exh: EXH,
}
impl MoveTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MoveTimeline").unwrap();let exd = game_data.read_excel_sheet("MoveTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MoveTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MoveTimelineRow { columns: row.columns.clone() }
}
}
pub struct MoveTimelineRow {
columns: Vec<ColumnData>,
}
impl MoveTimelineRow {
pub fn Idle(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MoveForward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MoveBack(&self) -> &ColumnData {
&self.columns[2]
}
pub fn MoveLeft(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MoveRight(&self) -> &ColumnData {
&self.columns[4]
}
pub fn MoveUp(&self) -> &ColumnData {
&self.columns[5]
}
pub fn MoveDown(&self) -> &ColumnData {
&self.columns[6]
}
pub fn MoveTurnLeft(&self) -> &ColumnData {
&self.columns[7]
}
pub fn MoveTurnRight(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Extra(&self) -> &ColumnData {
&self.columns[9]
}
}
