#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SpecialShopSheet {
exd: EXD,
exh: EXH,
}
impl SpecialShopSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("SpecialShop")?;let exd = game_data.read_excel_sheet("SpecialShop", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<SpecialShopRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SpecialShopRow { columns })
}
}
pub struct SpecialShopRow {
columns: Vec<ColumnData>,
}
impl SpecialShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RequiredContentFinderCondition(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CompleteText(&self) -> &ColumnData {
&self.columns[5]
}
pub fn NotCompleteText(&self) -> &ColumnData {
&self.columns[6]
}
pub fn RequiredFestival(&self) -> &ColumnData {
&self.columns[7]
}
pub fn RequiredFestivalPhase(&self) -> &ColumnData {
&self.columns[8]
}
pub fn UseCurrencyType(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[10]
}
pub fn RequiredContentFinderConditionComplete(&self) -> &ColumnData {
&self.columns[11]
}
}
