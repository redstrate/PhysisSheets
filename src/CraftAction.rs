#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CraftActionSheet {
exd: EXD,
exh: EXH,
}
impl CraftActionSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("CraftAction")?;let exd = game_data.read_excel_sheet("CraftAction", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<CraftActionRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CraftActionRow { columns })
}
}
pub struct CraftActionRow {
columns: Vec<ColumnData>,
}
impl CraftActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn QuestRequirement(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CRP(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BSM(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ARM(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GSM(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LTW(&self) -> &ColumnData {
&self.columns[7]
}
pub fn WVR(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ALC(&self) -> &ColumnData {
&self.columns[9]
}
pub fn CUL(&self) -> &ColumnData {
&self.columns[10]
}
pub fn AnimationStart(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AnimationEnd(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[13]
}
pub fn RequiredStatus(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Cost(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Specialist(&self) -> &ColumnData {
&self.columns[19]
}
}
