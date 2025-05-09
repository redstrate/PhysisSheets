#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringPoint {
exd: EXD,
exh: EXH,
}
impl GatheringPoint {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringPoint").unwrap();let exd = game_data.read_excel_sheet("GatheringPoint", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringPointRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringPointRow { columns: row.columns.clone() }
}
}
pub struct GatheringPointRow {
columns: Vec<ColumnData>,
}
impl GatheringPointRow {
pub fn GatheringPointBase(&self) -> &ColumnData {
&self.columns[0]
}
pub fn GatheringPointBonus(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GatheringSubCategory(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Count(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
}
