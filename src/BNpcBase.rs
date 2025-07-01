#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BNpcBaseSheet {
exd: EXD,
exh: EXH,
}
impl BNpcBaseSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BNpcBase")?;let exd = game_data.read_excel_sheet("BNpcBase", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BNpcBaseRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BNpcBaseRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BNpcBaseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcBaseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct BNpcBaseRow {
columns: Vec<ColumnData>,
}
impl BNpcBaseRow {
pub fn Scale(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ArrayEventHandler(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Behavior(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ModelChara(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BNpcCustomize(&self) -> &ColumnData {
&self.columns[4]
}
pub fn NpcEquip(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Special(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Battalion(&self) -> &ColumnData {
&self.columns[8]
}
pub fn LinkRace(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[10]
}
pub fn SEPack(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[12]
}
pub fn BNpcParts(&self) -> &ColumnData {
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
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[18]
}
pub fn IsOmnidirectional(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[20]
}
pub fn IsTargetLine(&self) -> &ColumnData {
&self.columns[21]
}
pub fn IsDisplayLevel(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[25]
}
}
