#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CutsceneMotion {
exd: EXD,
exh: EXH,
}
impl CutsceneMotion {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("CutsceneMotion")?;let exd = game_data.read_excel_sheet("CutsceneMotion", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<CutsceneMotionRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CutsceneMotionRow { columns })
}
}
pub struct CutsceneMotionRow {
columns: Vec<ColumnData>,
}
impl CutsceneMotionRow {
pub fn WALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SLOWWALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SLOWRUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BATTLEWALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[4]
}
pub fn BATTLERUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DASH_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TURN_CW90_FRAME(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TURN_CCW90_FRAME(&self) -> &ColumnData {
&self.columns[8]
}
pub fn TURN_CW180_FRAME(&self) -> &ColumnData {
&self.columns[9]
}
pub fn TURN_CCW180_FRAME(&self) -> &ColumnData {
&self.columns[10]
}
}
