#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GatheringSubCategory {
exd: EXD,
exh: EXH,
}
impl GatheringSubCategory {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringSubCategory").unwrap();let exd = game_data.read_excel_sheet("GatheringSubCategory", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringSubCategoryRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
GatheringSubCategoryRow { columns }
}
}
pub struct GatheringSubCategoryRow {
columns: Vec<ColumnData>,
}
impl GatheringSubCategoryRow {
pub fn FolkloreBook(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Division(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GatheringType(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
}
