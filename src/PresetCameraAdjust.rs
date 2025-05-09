#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PresetCameraAdjust {
exd: EXD,
exh: EXH,
}
impl PresetCameraAdjust {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PresetCameraAdjust").unwrap();let exd = game_data.read_excel_sheet("PresetCameraAdjust", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PresetCameraAdjustRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PresetCameraAdjustRow { columns: row.columns.clone() }
}
}
pub struct PresetCameraAdjustRow {
columns: Vec<ColumnData>,
}
impl PresetCameraAdjustRow {
pub fn Hyur_M(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Hyur_F(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Elezen_M(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Elezen_F(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Lalafell_M(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Lalafell_F(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Miqote_M(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Miqote_F(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Roe_M(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Roe_F(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Hrothgar_M(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Hrothgar_F(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Viera_M(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Viera_F(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[15]
}
}
