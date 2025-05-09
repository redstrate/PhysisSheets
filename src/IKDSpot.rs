#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct IKDSpot {
exd: EXD,
exh: EXH,
}
impl IKDSpot {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("IKDSpot").unwrap();let exd = game_data.read_excel_sheet("IKDSpot", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> IKDSpotRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
IKDSpotRow { columns: row.columns.clone() }
}
}
pub struct IKDSpotRow {
columns: Vec<ColumnData>,
}
impl IKDSpotRow {
pub fn SpotMain(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SpotSub(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[2]
}
}
