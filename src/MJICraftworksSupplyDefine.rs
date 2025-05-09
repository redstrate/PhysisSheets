#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MJICraftworksSupplyDefine {
exd: EXD,
exh: EXH,
}
impl MJICraftworksSupplyDefine {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICraftworksSupplyDefine").unwrap();let exd = game_data.read_excel_sheet("MJICraftworksSupplyDefine", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICraftworksSupplyDefineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
MJICraftworksSupplyDefineRow { columns }
}
}
pub struct MJICraftworksSupplyDefineRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksSupplyDefineRow {
pub fn Ratio(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Supply(&self) -> &ColumnData {
&self.columns[1]
}
}
