#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AnimaWeapon5TradeItemSheet {
exd: EXD,
exh: EXH,
}
impl AnimaWeapon5TradeItemSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("AnimaWeapon5TradeItem")?;let exd = game_data.read_excel_sheet("AnimaWeapon5TradeItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<AnimaWeapon5TradeItemRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AnimaWeapon5TradeItemRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<AnimaWeapon5TradeItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AnimaWeapon5TradeItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct AnimaWeapon5TradeItemRow {
columns: Vec<ColumnData>,
}
impl AnimaWeapon5TradeItemRow {
pub fn CrystalSand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> [&ColumnData; 8] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ReceiveQuantity(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Quantity(&self) -> [&ColumnData; 8] {
[&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],]
}
pub fn Category(&self) -> &ColumnData {
&self.columns[19]
}
pub fn IsHQ(&self) -> [&ColumnData; 8] {
[&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],]
}
}
