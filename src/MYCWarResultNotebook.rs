#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MYCWarResultNotebook {
exd: EXD,
exh: EXH,
}
impl MYCWarResultNotebook {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MYCWarResultNotebook").unwrap();let exd = game_data.read_excel_sheet("MYCWarResultNotebook", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MYCWarResultNotebookRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MYCWarResultNotebookRow { columns: row.columns.clone() }
}
}
pub struct MYCWarResultNotebookRow {
columns: Vec<ColumnData>,
}
impl MYCWarResultNotebookRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NameJP(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Link(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Rarity(&self) -> &ColumnData {
&self.columns[10]
}
}
