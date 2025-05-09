#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct FCRankSheet {
exd: EXD,
exh: EXH,
}
impl FCRankSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("FCRank")?;let exd = game_data.read_excel_sheet("FCRank", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<FCRankRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(FCRankRow { columns })
}
}
pub struct FCRankRow {
columns: Vec<ColumnData>,
}
impl FCRankRow {
pub fn NextPoint(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CurrentPoint(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Rights(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn FCActionActiveNum(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FCActionStockNum(&self) -> &ColumnData {
&self.columns[6]
}
pub fn FCChestCompartments(&self) -> &ColumnData {
&self.columns[7]
}
}
