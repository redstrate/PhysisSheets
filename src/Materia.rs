#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Materia {
exd: EXD,
exh: EXH,
}
impl Materia {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Materia").unwrap();let exd = game_data.read_excel_sheet("Materia", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MateriaRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MateriaRow { columns: row.columns.clone() }
}
}
pub struct MateriaRow {
columns: Vec<ColumnData>,
}
impl MateriaRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Value(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BaseParam(&self) -> &ColumnData {
&self.columns[2]
}
}
