#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GlassesStyle {
exd: EXD,
exh: EXH,
}
impl GlassesStyle {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GlassesStyle").unwrap();let exd = game_data.read_excel_sheet("GlassesStyle", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GlassesStyleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GlassesStyleRow { columns: row.columns.clone() }
}
}
pub struct GlassesStyleRow {
columns: Vec<ColumnData>,
}
impl GlassesStyleRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown_70_4(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown_70_5(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown_70_6(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Glasses(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown_70_7(&self) -> &ColumnData {
&self.columns[12]
}
}
