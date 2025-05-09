#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ENpcResidentSheet {
exd: EXD,
exh: EXH,
}
impl ENpcResidentSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("ENpcResident")?;let exd = game_data.read_excel_sheet("ENpcResident", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<ENpcResidentRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ENpcResidentRow { columns })
}
}
pub struct ENpcResidentRow {
columns: Vec<ColumnData>,
}
impl ENpcResidentRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Title(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[4]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
}
