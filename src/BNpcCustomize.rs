#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BNpcCustomize {
exd: EXD,
exh: EXH,
}
impl BNpcCustomize {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BNpcCustomize").unwrap();let exd = game_data.read_excel_sheet("BNpcCustomize", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BNpcCustomizeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BNpcCustomizeRow { columns: row.columns.clone() }
}
}
pub struct BNpcCustomizeRow {
columns: Vec<ColumnData>,
}
impl BNpcCustomizeRow {
pub fn Race(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Gender(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BodyType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Height(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Tribe(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Face(&self) -> &ColumnData {
&self.columns[5]
}
pub fn HairStyle(&self) -> &ColumnData {
&self.columns[6]
}
pub fn HairHighlight(&self) -> &ColumnData {
&self.columns[7]
}
pub fn SkinColor(&self) -> &ColumnData {
&self.columns[8]
}
pub fn EyeHeterochromia(&self) -> &ColumnData {
&self.columns[9]
}
pub fn HairColor(&self) -> &ColumnData {
&self.columns[10]
}
pub fn HairHighlightColor(&self) -> &ColumnData {
&self.columns[11]
}
pub fn FacialFeature(&self) -> &ColumnData {
&self.columns[12]
}
pub fn FacialFeatureColor(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Eyebrows(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EyeColor(&self) -> &ColumnData {
&self.columns[15]
}
pub fn EyeShape(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Nose(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Jaw(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Mouth(&self) -> &ColumnData {
&self.columns[19]
}
pub fn LipColor(&self) -> &ColumnData {
&self.columns[20]
}
pub fn BustOrTone1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ExtraFeature1(&self) -> &ColumnData {
&self.columns[22]
}
pub fn ExtraFeature2OrBust(&self) -> &ColumnData {
&self.columns[23]
}
pub fn FacePaint(&self) -> &ColumnData {
&self.columns[24]
}
pub fn FacePaintColor(&self) -> &ColumnData {
&self.columns[25]
}
}
