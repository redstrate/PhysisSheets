#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct HousingUnitedExterior {
exd: EXD,
exh: EXH,
}
impl HousingUnitedExterior {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("HousingUnitedExterior")?;let exd = game_data.read_excel_sheet("HousingUnitedExterior", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<HousingUnitedExteriorRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HousingUnitedExteriorRow { columns })
}
}
pub struct HousingUnitedExteriorRow {
columns: Vec<ColumnData>,
}
impl HousingUnitedExteriorRow {
pub fn Roof(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Walls(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Windows(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Door(&self) -> &ColumnData {
&self.columns[3]
}
pub fn OptionalRoof(&self) -> &ColumnData {
&self.columns[4]
}
pub fn OptionalWall(&self) -> &ColumnData {
&self.columns[5]
}
pub fn OptionalSignboard(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Fence(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PlotSize(&self) -> &ColumnData {
&self.columns[8]
}
}
