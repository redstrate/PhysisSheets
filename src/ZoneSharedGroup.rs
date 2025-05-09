#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ZoneSharedGroup {
exd: EXD,
exh: EXH,
}
impl ZoneSharedGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ZoneSharedGroup").unwrap();let exd = game_data.read_excel_sheet("ZoneSharedGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ZoneSharedGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
ZoneSharedGroupRow { columns }
}
}
pub struct ZoneSharedGroupRow {
columns: Vec<ColumnData>,
}
impl ZoneSharedGroupRow {
pub fn LGBSharedGroup(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Quest1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quest2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Quest3(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Quest4(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Quest5(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Seq0(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Seq1(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Seq2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Seq3(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Seq4(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Seq5(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[28]
}
}
