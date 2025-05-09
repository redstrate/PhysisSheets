#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GatheringLeveSheet {
exd: EXD,
exh: EXH,
}
impl GatheringLeveSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("GatheringLeve")?;let exd = game_data.read_excel_sheet("GatheringLeve", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<GatheringLeveRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GatheringLeveRow { columns })
}
}
pub struct GatheringLeveRow {
columns: Vec<ColumnData>,
}
impl GatheringLeveRow {
pub fn Route(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[2]
}
pub fn BNpcEntry(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RequiredItemQuantity(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ItemNumber(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Varient(&self) -> &ColumnData {
&self.columns[7]
}
pub fn UseSecondaryTool(&self) -> &ColumnData {
&self.columns[8]
}
}
