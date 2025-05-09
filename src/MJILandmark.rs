#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJILandmark {
exd: EXD,
exh: EXH,
}
impl MJILandmark {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJILandmark").unwrap();let exd = game_data.read_excel_sheet("MJILandmark", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJILandmarkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJILandmarkRow { columns: row.columns.clone() }
}
}
pub struct MJILandmarkRow {
columns: Vec<ColumnData>,
}
impl MJILandmarkRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SGB(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Material(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Amount(&self) -> &ColumnData {
&self.columns[18]
}
}
