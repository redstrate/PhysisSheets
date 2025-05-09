#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingMapMarkerInfo {
exd: EXD,
exh: EXH,
}
impl HousingMapMarkerInfo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingMapMarkerInfo").unwrap();let exd = game_data.read_excel_sheet("HousingMapMarkerInfo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingMapMarkerInfoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingMapMarkerInfoRow { columns: row.columns.clone() }
}
}
pub struct HousingMapMarkerInfoRow {
columns: Vec<ColumnData>,
}
impl HousingMapMarkerInfoRow {
pub fn X(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Y(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Z(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[4]
}
}
