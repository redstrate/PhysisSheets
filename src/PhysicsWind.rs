#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct PhysicsWindSheet {
exd: EXD,
exh: EXH,
}
impl PhysicsWindSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("PhysicsWind")?;let exd = game_data.read_excel_sheet("PhysicsWind", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<PhysicsWindRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(PhysicsWindRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<PhysicsWindRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PhysicsWindRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct PhysicsWindRow {
columns: Vec<ColumnData>,
}
impl PhysicsWindRow {
pub fn Threshold(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Amplitude(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AmplitudeFrequency(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PowerMin(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PowerMax(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PowerFrequency(&self) -> &ColumnData {
&self.columns[5]
}
}
