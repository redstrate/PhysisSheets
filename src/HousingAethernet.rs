#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingAethernet {
exd: EXD,
exh: EXH,
}
impl HousingAethernet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingAethernet").unwrap();let exd = game_data.read_excel_sheet("HousingAethernet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingAethernetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingAethernetRow { columns: row.columns.clone() }
}
}
pub struct HousingAethernetRow {
columns: Vec<ColumnData>,
}
impl HousingAethernetRow {
pub fn Level(&self) -> &ColumnData {
&self.columns[0]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[3]
}
}
