#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Skirmish {
exd: EXD,
exh: EXH,
}
impl Skirmish {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Skirmish").unwrap();let exd = game_data.read_excel_sheet("Skirmish", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SkirmishRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SkirmishRow { columns: row.columns.clone() }
}
}
pub struct SkirmishRow {
columns: Vec<ColumnData>,
}
impl SkirmishRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
