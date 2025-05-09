#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIVillageAppearanceSG {
exd: EXD,
exh: EXH,
}
impl MJIVillageAppearanceSG {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIVillageAppearanceSG").unwrap();let exd = game_data.read_excel_sheet("MJIVillageAppearanceSG", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIVillageAppearanceSGRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIVillageAppearanceSGRow { columns: row.columns.clone() }
}
}
pub struct MJIVillageAppearanceSGRow {
columns: Vec<ColumnData>,
}
impl MJIVillageAppearanceSGRow {
pub fn VillageAppearanceData(&self) -> &ColumnData {
&self.columns[0]
}
}
