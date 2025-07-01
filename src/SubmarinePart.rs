#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SubmarinePartSheet {
exd: EXD,
exh: EXH,
}
impl SubmarinePartSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("SubmarinePart")?;let exd = game_data.read_excel_sheet("SubmarinePart", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<SubmarinePartRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SubmarinePartRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<SubmarinePartRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SubmarinePartRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct SubmarinePartRow {
columns: Vec<ColumnData>,
}
impl SubmarinePartRow {
pub fn Class(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Surveillance(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Retrieval(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Speed(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Range(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Favor(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Components(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RepairMaterials(&self) -> &ColumnData {
&self.columns[9]
}
}
