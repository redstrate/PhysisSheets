#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIAnimals {
exd: EXD,
exh: EXH,
}
impl MJIAnimals {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIAnimals").unwrap();let exd = game_data.read_excel_sheet("MJIAnimals", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIAnimalsRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIAnimalsRow { columns: row.columns.clone() }
}
}
pub struct MJIAnimalsRow {
columns: Vec<ColumnData>,
}
impl MJIAnimalsRow {
pub fn BNpcBase(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Size(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
}
