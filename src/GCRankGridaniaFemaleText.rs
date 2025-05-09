#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GCRankGridaniaFemaleText {
exd: EXD,
exh: EXH,
}
impl GCRankGridaniaFemaleText {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GCRankGridaniaFemaleText").unwrap();let exd = game_data.read_excel_sheet("GCRankGridaniaFemaleText", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GCRankGridaniaFemaleTextRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
GCRankGridaniaFemaleTextRow { columns }
}
}
pub struct GCRankGridaniaFemaleTextRow {
columns: Vec<ColumnData>,
}
impl GCRankGridaniaFemaleTextRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NameRank(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[5]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[9]
}
}
