#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BenchmarkOverrideEquipment {
exd: EXD,
exh: EXH,
}
impl BenchmarkOverrideEquipment {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BenchmarkOverrideEquipment")?;let exd = game_data.read_excel_sheet("BenchmarkOverrideEquipment", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<BenchmarkOverrideEquipmentRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BenchmarkOverrideEquipmentRow { columns })
}
}
pub struct BenchmarkOverrideEquipmentRow {
columns: Vec<ColumnData>,
}
impl BenchmarkOverrideEquipmentRow {
pub fn ModelMainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ModelOffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ModelHead(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ModelBody(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ModelHands(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ModelLegs(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ModelFeet(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ModelEars(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ModelNeck(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ModelWrists(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ModelLeftRing(&self) -> &ColumnData {
&self.columns[13]
}
pub fn ModelRightRing(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[15]
}
pub fn DyeMainHand(&self) -> &ColumnData {
&self.columns[16]
}
pub fn DyeOffHand(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[18]
}
pub fn DyeHead(&self) -> &ColumnData {
&self.columns[19]
}
pub fn DyeBody(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DyeHands(&self) -> &ColumnData {
&self.columns[21]
}
pub fn DyeLegs(&self) -> &ColumnData {
&self.columns[22]
}
pub fn DyeFeet(&self) -> &ColumnData {
&self.columns[23]
}
pub fn DyeEars(&self) -> &ColumnData {
&self.columns[24]
}
pub fn DyeNeck(&self) -> &ColumnData {
&self.columns[25]
}
pub fn DyeWrists(&self) -> &ColumnData {
&self.columns[26]
}
pub fn DyeLeftRing(&self) -> &ColumnData {
&self.columns[27]
}
pub fn DyeRightRing(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[29]
}
}
