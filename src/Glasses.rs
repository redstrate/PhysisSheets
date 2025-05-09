#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Glasses {
exd: EXD,
exh: EXH,
}
impl Glasses {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Glasses")?;let exd = game_data.read_excel_sheet("Glasses", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<GlassesRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GlassesRow { columns })
}
}
pub struct GlassesRow {
columns: Vec<ColumnData>,
}
impl GlassesRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown_70_4(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown_70_5(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown_70_6(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown_70_7(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown_70_8(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Style(&self) -> &ColumnData {
&self.columns[13]
}
}
