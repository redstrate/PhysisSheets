#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ModelSkeleton {
exd: EXD,
exh: EXH,
}
impl ModelSkeleton {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ModelSkeleton").unwrap();let exd = game_data.read_excel_sheet("ModelSkeleton", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ModelSkeletonRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ModelSkeletonRow { columns: row.columns.clone() }
}
}
pub struct ModelSkeletonRow {
columns: Vec<ColumnData>,
}
impl ModelSkeletonRow {
pub fn Radius(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[1]
}
pub fn VFXScale(&self) -> &ColumnData {
&self.columns[2]
}
pub fn FloatHeight(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FloatDown(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[12]
}
pub fn FloatUp(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[14]
}
pub fn LoopFlySE(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MotionBlendType(&self) -> &ColumnData {
&self.columns[16]
}
}
