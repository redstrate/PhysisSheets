#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FishingSpot {
exd: EXD,
exh: EXH,
}
impl FishingSpot {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FishingSpot").unwrap();let exd = game_data.read_excel_sheet("FishingSpot", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FishingSpotRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FishingSpotRow { columns: row.columns.clone() }
}
}
pub struct FishingSpotRow {
columns: Vec<ColumnData>,
}
impl FishingSpotRow {
pub fn BigFishOnReach(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BigFishOnEnd(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlaceNameMain(&self) -> &ColumnData {
&self.columns[5]
}
pub fn PlaceNameSub(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Radius(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[9]
}
pub fn X(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Z(&self) -> &ColumnData {
&self.columns[11]
}
pub fn GatheringLevel(&self) -> &ColumnData {
&self.columns[12]
}
pub fn FishingSpotCategory(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Rare(&self) -> &ColumnData {
&self.columns[15]
}
}
