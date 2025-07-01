#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CompanionTransientSheet {
exd: EXD,
exh: EXH,
}
impl CompanionTransientSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("CompanionTransient")?;let exd = game_data.read_excel_sheet("CompanionTransient", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CompanionTransientRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CompanionTransientRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CompanionTransientRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanionTransientRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct CompanionTransientRow {
columns: Vec<ColumnData>,
}
impl CompanionTransientRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DescriptionEnhanced(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Tooltip(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SpecialActionName(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SpecialActionDescription(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Attack(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Defense(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Speed(&self) -> &ColumnData {
&self.columns[7]
}
pub fn MinionSkillType(&self) -> &ColumnData {
&self.columns[8]
}
pub fn HasAreaAttack(&self) -> &ColumnData {
&self.columns[9]
}
pub fn StrengthGate(&self) -> &ColumnData {
&self.columns[10]
}
pub fn StrengthEye(&self) -> &ColumnData {
&self.columns[11]
}
pub fn StrengthShield(&self) -> &ColumnData {
&self.columns[12]
}
pub fn StrengthArcana(&self) -> &ColumnData {
&self.columns[13]
}
}
