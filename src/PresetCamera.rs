#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PresetCamera {
exd: EXD,
exh: EXH,
}
impl PresetCamera {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PresetCamera").unwrap();let exd = game_data.read_excel_sheet("PresetCamera", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PresetCameraRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PresetCameraRow { columns: row.columns.clone() }
}
}
pub struct PresetCameraRow {
columns: Vec<ColumnData>,
}
impl PresetCameraRow {
pub fn PosX(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PosY(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PosZ(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Elezen(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Lalafell(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Miqote(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Roe(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Hrothgar(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Viera(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Hyur_F(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Elezen_F(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Lalafell_F(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Miqote_F(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Roe_F(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Hrothgar_F(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Viera_F(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[17]
}
pub fn EID(&self) -> &ColumnData {
&self.columns[18]
}
}
