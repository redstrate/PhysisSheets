#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RelicNoteSheet {
exd: EXD,
exh: EXH,
}
impl RelicNoteSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("RelicNote")?;let exd = game_data.read_excel_sheet("RelicNote", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<RelicNoteRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RelicNoteRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<RelicNoteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RelicNoteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct RelicNoteRow {
columns: Vec<ColumnData>,
}
impl RelicNoteRow {
pub fn EventItem(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MonsterNoteTargetCommon(&self) -> [&ColumnData; 10] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],]
}
pub fn MonsterNoteTargetNM(&self) -> [&ColumnData; 3] {
[&self.columns[11],&self.columns[12],&self.columns[13],]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Fate(&self) -> [&ColumnData; 3] {
[&self.columns[15],&self.columns[16],&self.columns[17],]
}
pub fn PlaceNameFate(&self) -> [&ColumnData; 3] {
[&self.columns[18],&self.columns[19],&self.columns[20],]
}
pub fn Leve(&self) -> [&ColumnData; 3] {
[&self.columns[21],&self.columns[22],&self.columns[23],]
}
pub fn MonsterCount(&self) -> [&ColumnData; 10] {
[&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],]
}
}
