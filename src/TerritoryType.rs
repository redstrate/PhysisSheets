#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TerritoryType {
exd: EXD,
exh: EXH,
}
impl TerritoryType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TerritoryType").unwrap();let exd = game_data.read_excel_sheet("TerritoryType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TerritoryTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TerritoryTypeRow { columns: row.columns.clone() }
}
}
pub struct TerritoryTypeRow {
columns: Vec<ColumnData>,
}
impl TerritoryTypeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Bg(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ArrayEventHandler(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PlaceNameRegionIcon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PlaceNameIcon(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Aetheryte(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FixedTime(&self) -> &ColumnData {
&self.columns[6]
}
pub fn PlaceNameRegion(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PlaceNameZone(&self) -> &ColumnData {
&self.columns[8]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[11]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[12]
}
pub fn QuestBattle(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Resident(&self) -> &ColumnData {
&self.columns[14]
}
pub fn NotoriousMonsterTerritory(&self) -> &ColumnData {
&self.columns[15]
}
pub fn BattalionMode(&self) -> &ColumnData {
&self.columns[16]
}
pub fn LoadingImage(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ExclusiveType(&self) -> &ColumnData {
&self.columns[18]
}
pub fn TerritoryIntendedUse(&self) -> &ColumnData {
&self.columns[19]
}
pub fn WeatherRate(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ExVersion(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[24]
}
pub fn AetherCurrentCompFlgSet(&self) -> &ColumnData {
&self.columns[25]
}
pub fn MountSpeed(&self) -> &ColumnData {
&self.columns[26]
}
pub fn IndividualWeather(&self) -> &ColumnData {
&self.columns[27]
}
pub fn AchievementIndex(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[30]
}
pub fn PCSearch(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Stealth(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Mount(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[34]
}
pub fn IsPvpZone(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[43]
}
}
