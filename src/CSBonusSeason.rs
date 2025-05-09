#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CSBonusSeason {
exd: EXD,
exh: EXH,
}
impl CSBonusSeason {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusSeason").unwrap();let exd = game_data.read_excel_sheet("CSBonusSeason", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusSeasonRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CSBonusSeasonRow { columns: row.columns.clone() }
}
}
pub struct CSBonusSeasonRow {
columns: Vec<ColumnData>,
}
impl CSBonusSeasonRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Category0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Category1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Category2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Category3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Text0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Text1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[12]
}
}
