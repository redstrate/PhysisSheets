#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct DawnGrowMemberSheet {
exd: EXD,
exh: EXH,
}
impl DawnGrowMemberSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("DawnGrowMember")?;let exd = game_data.read_excel_sheet("DawnGrowMember", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<DawnGrowMemberRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(DawnGrowMemberRow { columns })
}
}
pub struct DawnGrowMemberRow {
columns: Vec<ColumnData>,
}
impl DawnGrowMemberRow {
pub fn SelectImage(&self) -> [&ColumnData; 4] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],]
}
pub fn PortraitImage(&self) -> [&ColumnData; 4] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn Class(&self) -> &ColumnData {
&self.columns[8]
}
}
