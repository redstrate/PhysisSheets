#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RetainerTaskNormalSheet {
exd: EXD,
exh: EXH,
}
impl RetainerTaskNormalSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("RetainerTaskNormal")?;let exd = game_data.read_excel_sheet("RetainerTaskNormal", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<RetainerTaskNormalRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RetainerTaskNormalRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<RetainerTaskNormalRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RetainerTaskNormalRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct RetainerTaskNormalRow {
columns: Vec<ColumnData>,
}
impl RetainerTaskNormalRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GatheringLog(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FishingLog(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quantity(&self) -> [&ColumnData; 5] {
[&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
}
