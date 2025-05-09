#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Ornament {
exd: EXD,
exh: EXH,
}
impl Ornament {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Ornament").unwrap();let exd = game_data.read_excel_sheet("Ornament", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> OrnamentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
OrnamentRow { columns }
}
}
pub struct OrnamentRow {
columns: Vec<ColumnData>,
}
impl OrnamentRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Model(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Transient(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[12]
}
pub fn AttachmentPoint(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[15]
}
}
