#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Map {
exd: EXD,
exh: EXH,
}
impl Map {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Map").unwrap();let exd = game_data.read_excel_sheet("Map", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MapRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MapRow { columns: row.columns.clone() }
}
}
pub struct MapRow {
columns: Vec<ColumnData>,
}
impl MapRow {
pub fn Id(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DiscoveryFlag(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MapMarkerRange(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SizeFactor(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PlaceNameRegion(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[5]
}
pub fn PlaceNameSub(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[7]
}
pub fn OffsetX(&self) -> &ColumnData {
&self.columns[8]
}
pub fn OffsetY(&self) -> &ColumnData {
&self.columns[9]
}
pub fn DiscoveryIndex(&self) -> &ColumnData {
&self.columns[10]
}
pub fn MapCondition(&self) -> &ColumnData {
&self.columns[11]
}
pub fn PriorityCategoryUI(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PriorityUI(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Hierarchy(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[16]
}
pub fn MapIndex(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DiscoveryArrayByte(&self) -> &ColumnData {
&self.columns[18]
}
pub fn IsEvent(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[20]
}
}
