#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BannerPreset {
exd: EXD,
exh: EXH,
}
impl BannerPreset {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BannerPreset").unwrap();let exd = game_data.read_excel_sheet("BannerPreset", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BannerPresetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BannerPresetRow { columns: row.columns.clone() }
}
}
pub struct BannerPresetRow {
columns: Vec<ColumnData>,
}
impl BannerPresetRow {
pub fn CameraPositionX(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CameraPositionY(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CameraPositionZ(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CameraTargetX(&self) -> &ColumnData {
&self.columns[3]
}
pub fn CameraTargetY(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CameraTargetZ(&self) -> &ColumnData {
&self.columns[5]
}
pub fn AnimationProgress(&self) -> &ColumnData {
&self.columns[6]
}
pub fn HeadDirectionX(&self) -> &ColumnData {
&self.columns[7]
}
pub fn HeadDirectionY(&self) -> &ColumnData {
&self.columns[8]
}
pub fn EyeDirectionX(&self) -> &ColumnData {
&self.columns[9]
}
pub fn EyeDirectionY(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Expression(&self) -> &ColumnData {
&self.columns[11]
}
pub fn BannerTimeline(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ImageRotation(&self) -> &ColumnData {
&self.columns[13]
}
pub fn DirectionalLightingVerticalAngle(&self) -> &ColumnData {
&self.columns[14]
}
pub fn DirectionalLightingHorizontalAngle(&self) -> &ColumnData {
&self.columns[15]
}
pub fn CameraZoom(&self) -> &ColumnData {
&self.columns[16]
}
pub fn BannerDesignPreset(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DirectionalLightingColorRed(&self) -> &ColumnData {
&self.columns[18]
}
pub fn DirectionalLightingColorGreen(&self) -> &ColumnData {
&self.columns[19]
}
pub fn DirectionalLightingColorBlue(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DirectionalLightingBrightness(&self) -> &ColumnData {
&self.columns[21]
}
pub fn AmbientLightingColorRed(&self) -> &ColumnData {
&self.columns[22]
}
pub fn AmbientLightingColorGreen(&self) -> &ColumnData {
&self.columns[23]
}
pub fn AmbientLightingColorBlue(&self) -> &ColumnData {
&self.columns[24]
}
pub fn AmbientLightingBrightness(&self) -> &ColumnData {
&self.columns[25]
}
}
