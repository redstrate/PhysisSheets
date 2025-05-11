#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct HowToPageSheet {
exd: EXD,
exh: EXH,
}
impl HowToPageSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("HowToPage")?;let exd = game_data.read_excel_sheet("HowToPage", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<HowToPageRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HowToPageRow { columns })
}
}
pub struct HowToPageRow {
columns: Vec<ColumnData>,
}
impl HowToPageRow {
pub fn Text(&self) -> [&ColumnData; 3] {
[&self.columns[0],&self.columns[1],&self.columns[2],]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IconType(&self) -> &ColumnData {
&self.columns[5]
}
pub fn TextType(&self) -> &ColumnData {
&self.columns[6]
}
}
