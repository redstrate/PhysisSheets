#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PvPSelectTrait {
exd: EXD,
exh: EXH,
}
impl PvPSelectTrait {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PvPSelectTrait").unwrap();let exd = game_data.read_excel_sheet("PvPSelectTrait", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PvPSelectTraitRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PvPSelectTraitRow { columns: row.columns.clone() }
}
}
pub struct PvPSelectTraitRow {
columns: Vec<ColumnData>,
}
impl PvPSelectTraitRow {
pub fn Effect(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Value(&self) -> &ColumnData {
&self.columns[2]
}
}
