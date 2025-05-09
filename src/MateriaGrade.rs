#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MateriaGrade {
exd: EXD,
exh: EXH,
}
impl MateriaGrade {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MateriaGrade").unwrap();let exd = game_data.read_excel_sheet("MateriaGrade", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MateriaGradeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MateriaGradeRow { columns: row.columns.clone() }
}
}
pub struct MateriaGradeRow {
columns: Vec<ColumnData>,
}
impl MateriaGradeRow {
pub fn MeldFee(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ReturnRate(&self) -> &ColumnData {
&self.columns[1]
}
pub fn OvermeldNQPercent(&self) -> &ColumnData {
&self.columns[2]
}
pub fn OvermeldHQPercent(&self) -> &ColumnData {
&self.columns[3]
}
}
