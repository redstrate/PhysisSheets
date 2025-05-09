#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RetainerTaskSheet {
exd: EXD,
exh: EXH,
}
impl RetainerTaskSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("RetainerTask")?;let exd = game_data.read_excel_sheet("RetainerTask", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<RetainerTaskRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RetainerTaskRow { columns })
}
}
pub struct RetainerTaskRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskRow {
pub fn Experience(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RetainerTaskParameter(&self) -> &ColumnData {
&self.columns[2]
}
pub fn VentureCost(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MaxTimemin(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RequiredItemLevel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn RequiredGathering(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Task(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RetainerLevel(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ConditionParam0(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ConditionParam1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn IsRandom(&self) -> &ColumnData {
&self.columns[13]
}
}
