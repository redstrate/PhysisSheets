#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Relic3 {
exd: EXD,
exh: EXH,
}
impl Relic3 {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Relic3").unwrap();let exd = game_data.read_excel_sheet("Relic3", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> Relic3Row {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Relic3Row { columns }
}
}
pub struct Relic3Row {
columns: Vec<ColumnData>,
}
impl Relic3Row {
pub fn ItemAnimus(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemScroll(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ItemNovus(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MateriaLimit(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
}
