#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MotionTimelineBlendTable {
exd: EXD,
exh: EXH,
}
impl MotionTimelineBlendTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MotionTimelineBlendTable").unwrap();let exd = game_data.read_excel_sheet("MotionTimelineBlendTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MotionTimelineBlendTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MotionTimelineBlendTableRow { columns: row.columns.clone() }
}
}
pub struct MotionTimelineBlendTableRow {
columns: Vec<ColumnData>,
}
impl MotionTimelineBlendTableRow {
pub fn DestBlendGroup(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SrcBlendGroup(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BlendFrame_PC(&self) -> &ColumnData {
&self.columns[2]
}
pub fn BlendFram_TypeA(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BlendFram_TypeB(&self) -> &ColumnData {
&self.columns[4]
}
pub fn BlendFram_TypeC(&self) -> &ColumnData {
&self.columns[5]
}
}
