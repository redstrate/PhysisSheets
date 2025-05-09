#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct HousingYardObject {
exd: EXD,
exh: EXH,
}
impl HousingYardObject {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("HousingYardObject")?;let exd = game_data.read_excel_sheet("HousingYardObject", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<HousingYardObjectRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HousingYardObjectRow { columns })
}
}
pub struct HousingYardObjectRow {
columns: Vec<ColumnData>,
}
impl HousingYardObjectRow {
pub fn UsageParameter(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CustomTalk(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ModelKey(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HousingItemCategory(&self) -> &ColumnData {
&self.columns[4]
}
pub fn UsageType(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[9]
}
pub fn DestroyOnRemoval(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[14]
}
}
