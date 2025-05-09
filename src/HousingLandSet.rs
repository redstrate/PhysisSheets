#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingLandSet {
exd: EXD,
exh: EXH,
}
impl HousingLandSet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingLandSet").unwrap();let exd = game_data.read_excel_sheet("HousingLandSet", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingLandSetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingLandSetRow { columns: row.columns.clone() }
}
}
pub struct HousingLandSetRow {
columns: Vec<ColumnData>,
}
impl HousingLandSetRow {
pub fn LandSet(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnknownRange1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn UnknownRange2(&self) -> &ColumnData {
&self.columns[2]
}
}
