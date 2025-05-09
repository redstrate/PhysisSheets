#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Description {
exd: EXD,
exh: EXH,
}
impl Description {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Description").unwrap();let exd = game_data.read_excel_sheet("Description", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DescriptionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DescriptionRow { columns: row.columns.clone() }
}
}
pub struct DescriptionRow {
columns: Vec<ColumnData>,
}
impl DescriptionRow {
pub fn TextLong(&self) -> &ColumnData {
&self.columns[0]
}
pub fn TextShort(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TextCommentary(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Section(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[6]
}
}
