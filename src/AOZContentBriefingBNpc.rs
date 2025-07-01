#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AOZContentBriefingBNpcSheet {
exd: EXD,
exh: EXH,
}
impl AOZContentBriefingBNpcSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("AOZContentBriefingBNpc")?;let exd = game_data.read_excel_sheet("AOZContentBriefingBNpc", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<AOZContentBriefingBNpcRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AOZContentBriefingBNpcRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<AOZContentBriefingBNpcRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AOZContentBriefingBNpcRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct AOZContentBriefingBNpcRow {
columns: Vec<ColumnData>,
}
impl AOZContentBriefingBNpcRow {
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn TargetSmall(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TargetLarge(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Endurance(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Fire(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Ice(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Wind(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Earth(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Thunder(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Water(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Slashing(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Piercing(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Blunt(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Magic(&self) -> &ColumnData {
&self.columns[13]
}
pub fn HideStats(&self) -> &ColumnData {
&self.columns[14]
}
pub fn SlowVuln(&self) -> &ColumnData {
&self.columns[15]
}
pub fn PetrificationVuln(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ParalysisVuln(&self) -> &ColumnData {
&self.columns[17]
}
pub fn InterruptionVuln(&self) -> &ColumnData {
&self.columns[18]
}
pub fn BlindVuln(&self) -> &ColumnData {
&self.columns[19]
}
pub fn StunVuln(&self) -> &ColumnData {
&self.columns[20]
}
pub fn SleepVuln(&self) -> &ColumnData {
&self.columns[21]
}
pub fn BindVuln(&self) -> &ColumnData {
&self.columns[22]
}
pub fn HeavyVuln(&self) -> &ColumnData {
&self.columns[23]
}
pub fn FlatOrDeathVuln(&self) -> &ColumnData {
&self.columns[24]
}
}
