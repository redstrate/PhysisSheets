#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BNpcCustomizeSheet {
exd: EXD,
exh: EXH,
}
impl BNpcCustomizeSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BNpcCustomize")?;let exd = game_data.read_excel_sheet("BNpcCustomize", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<BNpcCustomizeRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BNpcCustomizeRow { columns })
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
