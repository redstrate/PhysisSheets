#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingEmploymentNpcRace {
exd: EXD,
exh: EXH,
}
impl HousingEmploymentNpcRace {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingEmploymentNpcRace").unwrap();let exd = game_data.read_excel_sheet("HousingEmploymentNpcRace", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingEmploymentNpcRaceRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingEmploymentNpcRaceRow { columns: row.columns.clone() }
}
}
pub struct HousingEmploymentNpcRaceRow {
columns: Vec<ColumnData>,
}
impl HousingEmploymentNpcRaceRow {
pub fn Race(&self) -> &ColumnData {
&self.columns[0]
}
}
