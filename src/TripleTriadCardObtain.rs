#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TripleTriadCardObtainSheet {
exd: EXD,
exh: EXH,
}
impl TripleTriadCardObtainSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("TripleTriadCardObtain")?;let exd = game_data.read_excel_sheet("TripleTriadCardObtain", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<TripleTriadCardObtainRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TripleTriadCardObtainRow { columns })
}
}
pub struct TripleTriadCardObtainRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCardObtainRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Text(&self) -> &ColumnData {
&self.columns[1]
}
}
