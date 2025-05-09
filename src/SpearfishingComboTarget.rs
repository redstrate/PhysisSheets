#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SpearfishingComboTarget {
exd: EXD,
exh: EXH,
}
impl SpearfishingComboTarget {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SpearfishingComboTarget").unwrap();let exd = game_data.read_excel_sheet("SpearfishingComboTarget", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SpearfishingComboTargetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
SpearfishingComboTargetRow { columns }
}
}
pub struct SpearfishingComboTargetRow {
columns: Vec<ColumnData>,
}
impl SpearfishingComboTargetRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[2]
}
}
