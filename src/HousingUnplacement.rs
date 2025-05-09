#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingUnplacement {
exd: EXD,
exh: EXH,
}
impl HousingUnplacement {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingUnplacement").unwrap();let exd = game_data.read_excel_sheet("HousingUnplacement", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingUnplacementRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingUnplacementRow { columns: row.columns.clone() }
}
}
pub struct HousingUnplacementRow {
columns: Vec<ColumnData>,
}
impl HousingUnplacementRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
