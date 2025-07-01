#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BannerConditionSheet {
exd: EXD,
exh: EXH,
}
impl BannerConditionSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BannerCondition")?;let exd = game_data.read_excel_sheet("BannerCondition", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<BannerConditionRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BannerConditionRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<BannerConditionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BannerConditionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct BannerConditionRow {
columns: Vec<ColumnData>,
}
impl BannerConditionRow {
pub fn UnlockCriteria1(&self) -> [&ColumnData; 6] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],]
}
pub fn UnlockCriteria2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn UnlockCriteria3(&self) -> &ColumnData {
&self.columns[7]
}
pub fn UnlockCriteria4(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Prerequisite(&self) -> &ColumnData {
&self.columns[10]
}
pub fn UnlockType1(&self) -> &ColumnData {
&self.columns[11]
}
pub fn UnlockType2(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PrerequisiteType(&self) -> &ColumnData {
&self.columns[13]
}
pub fn UnlockHint(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[15]
}
}
