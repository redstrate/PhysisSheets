#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Credit {
exd: EXD,
exh: EXH,
}
impl Credit {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Credit").unwrap();let exd = game_data.read_excel_sheet("Credit", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CreditRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CreditRow { columns: row.columns.clone() }
}
}
pub struct CreditRow {
columns: Vec<ColumnData>,
}
impl CreditRow {
pub fn Roles1(&self) -> &ColumnData {
&self.columns[0]
}
pub fn JapaneseCast1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn EnglishCast1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn FrenchCast1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GermanCast1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Roles2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn JapaneseCast2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn EnglishCast2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn FrenchCast2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn GermanCast2(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[10]
}
}
