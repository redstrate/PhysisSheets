#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingPlacement {
exd: EXD,
exh: EXH,
}
impl HousingPlacement {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingPlacement").unwrap();let exd = game_data.read_excel_sheet("HousingPlacement", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingPlacementRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingPlacementRow { columns: row.columns.clone() }
}
}
pub struct HousingPlacementRow {
columns: Vec<ColumnData>,
}
impl HousingPlacementRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
}
