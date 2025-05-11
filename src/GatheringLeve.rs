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
pub fn Route(&self) -> [&ColumnData; 4] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],]
}
pub fn RequiredItem(&self) -> [&ColumnData; 4] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[8]
}
pub fn BNpcEntry(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Objective(&self) -> [&ColumnData; 2] {
[&self.columns[10],&self.columns[11],]
}
pub fn RequiredItemQuantity(&self) -> [&ColumnData; 4] {
[&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],]
}
pub fn ItemNumber(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Varient(&self) -> &ColumnData {
&self.columns[17]
}
pub fn UseSecondaryTool(&self) -> &ColumnData {
&self.columns[18]
}
}
