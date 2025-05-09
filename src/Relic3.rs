#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Relic3 {
exd: EXD,
exh: EXH,
}
impl Relic3 {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Relic3").unwrap();let exd = game_data.read_excel_sheet("Relic3", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> Relic3Row {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
Relic3Row { columns: row.columns.clone() }
}
}
pub struct Relic3Row {
columns: Vec<ColumnData>,
}
impl Relic3Row {
pub fn ItemAnimus(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemScroll(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ItemNovus(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MateriaLimit(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
}
