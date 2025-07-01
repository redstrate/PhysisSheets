#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MJIRecipeSheet {
exd: EXD,
exh: EXH,
}
impl MJIRecipeSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("MJIRecipe")?;let exd = game_data.read_excel_sheet("MJIRecipe", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<MJIRecipeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MJIRecipeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<MJIRecipeRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJIRecipeRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct MJIRecipeRow {
columns: Vec<ColumnData>,
}
impl MJIRecipeRow {
pub fn LogMessage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn KeyItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ItemPouch(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Yield(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Material(&self) -> [&ColumnData; 5] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],]
}
pub fn Amount(&self) -> [&ColumnData; 5] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[14]
}
}
