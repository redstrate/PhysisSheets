#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Relic {
exd: EXD,
exh: EXH,
}
impl Relic {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Relic").unwrap();let exd = game_data.read_excel_sheet("Relic", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RelicRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RelicRow { columns: row.columns.clone() }
}
}
pub struct RelicRow {
columns: Vec<ColumnData>,
}
impl RelicRow {
pub fn ItemAtma(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ItemAnimus(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Materia0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Materia1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Materia2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Materia3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn NoteMain0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn NoteSub0(&self) -> &ColumnData {
&self.columns[8]
}
pub fn NoteSelection10(&self) -> &ColumnData {
&self.columns[9]
}
pub fn NoteMain1(&self) -> &ColumnData {
&self.columns[10]
}
pub fn NoteSub1(&self) -> &ColumnData {
&self.columns[11]
}
pub fn NoteSelection1(&self) -> &ColumnData {
&self.columns[12]
}
pub fn NoteMain2(&self) -> &ColumnData {
&self.columns[13]
}
pub fn NoteSub2(&self) -> &ColumnData {
&self.columns[14]
}
pub fn NoteSelection3(&self) -> &ColumnData {
&self.columns[15]
}
}
