#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ZoneSharedGroupSheet {
exd: EXD,
exh: EXH,
}
impl ZoneSharedGroupSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("ZoneSharedGroup")?;let exd = game_data.read_excel_sheet("ZoneSharedGroup", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ZoneSharedGroupRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ZoneSharedGroupRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ZoneSharedGroupRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ZoneSharedGroupRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ZoneSharedGroupRow {
columns: Vec<ColumnData>,
}
impl ZoneSharedGroupRow {
pub fn LGBSharedGroup(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequirementRow(&self) -> [&ColumnData; 6] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn RequirementQuestSequence(&self) -> [&ColumnData; 6] {
[&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[14]
}
/// 1 = Quest
2 = Quest with specific Sequence
3 = AetherCurrent
4 = EurekaStoryProgress
5 = DomaStoryProgress

pub fn RequirementType(&self) -> [&ColumnData; 6] {
[&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[28]
}
}
