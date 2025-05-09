#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RelicItemSheet {
exd: EXD,
exh: EXH,
}
impl RelicItemSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("RelicItem")?;let exd = game_data.read_excel_sheet("RelicItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<RelicItemRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RelicItemRow { columns })
}
}
pub struct RelicItemRow {
columns: Vec<ColumnData>,
}
impl RelicItemRow {
pub fn GladiatorItem(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PugilistItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MarauderItem(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LancerItem(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ArcherItem(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ConjurerItem(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ThaumaturgeItem(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ArcanistSMNItem(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ArcanistSCHItem(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ShieldItem(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RogueItem(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[16]
}
}
