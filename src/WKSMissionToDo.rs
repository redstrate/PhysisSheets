#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct WKSMissionToDoSheet {
exd: EXD,
exh: EXH,
}
impl WKSMissionToDoSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("WKSMissionToDo")?;let exd = game_data.read_excel_sheet("WKSMissionToDo", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSMissionToDoRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WKSMissionToDoRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<WKSMissionToDoRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionToDoRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct WKSMissionToDoRow {
columns: Vec<ColumnData>,
}
impl WKSMissionToDoRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredItem(&self) -> [&ColumnData; 3] {
[&self.columns[3],&self.columns[4],&self.columns[5],]
}
pub fn RequiredItemQuantity(&self) -> [&ColumnData; 3] {
[&self.columns[6],&self.columns[7],&self.columns[8],]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[17]
}
pub fn MissionType(&self) -> &ColumnData {
&self.columns[18]
}
pub fn WKSMissionText(&self) -> &ColumnData {
&self.columns[19]
}
}
