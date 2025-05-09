#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ActionTimeline {
exd: EXD,
exh: EXH,
}
impl ActionTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ActionTimeline").unwrap();let exd = game_data.read_excel_sheet("ActionTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ActionTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
ActionTimelineRow { columns }
}
}
pub struct ActionTimelineRow {
columns: Vec<ColumnData>,
}
impl ActionTimelineRow {
pub fn Key(&self) -> &ColumnData {
&self.columns[0]
}
pub fn WeaponTimeline(&self) -> &ColumnData {
&self.columns[1]
}
pub fn KillUpper(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Priority(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Stance(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LookAtMode(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ActionTimelineIDMode(&self) -> &ColumnData {
&self.columns[9]
}
pub fn LoadType(&self) -> &ColumnData {
&self.columns[10]
}
pub fn StartAttach(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ResidentPap(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Pause(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Resident(&self) -> &ColumnData {
&self.columns[16]
}
pub fn IsMotionCanceledByMoving(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[19]
}
pub fn IsLoop(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[21]
}
}
