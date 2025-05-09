#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Leve {
exd: EXD,
exh: EXH,
}
impl Leve {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Leve").unwrap();let exd = game_data.read_excel_sheet("Leve", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LeveRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LeveRow { columns: row.columns.clone() }
}
}
pub struct LeveRow {
columns: Vec<ColumnData>,
}
impl LeveRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ExpFactor(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ExpReward(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GilReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LeveRewardItem(&self) -> &ColumnData {
&self.columns[5]
}
pub fn JournalGenre(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LevelLevemete(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LevelStart(&self) -> &ColumnData {
&self.columns[8]
}
pub fn LeveClient(&self) -> &ColumnData {
&self.columns[9]
}
pub fn LeveAssignmentType(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Town(&self) -> &ColumnData {
&self.columns[11]
}
pub fn PlaceNameStart(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PlaceNameIssued(&self) -> &ColumnData {
&self.columns[13]
}
pub fn PlaceNameStartZone(&self) -> &ColumnData {
&self.columns[14]
}
pub fn IconCityState(&self) -> &ColumnData {
&self.columns[15]
}
pub fn DataId(&self) -> &ColumnData {
&self.columns[16]
}
pub fn IconIssuer(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[18]
}
pub fn FishingSpot(&self) -> &ColumnData {
&self.columns[19]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[22]
}
pub fn AllowanceCost(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[24]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[25]
}
pub fn MaxDifficulty(&self) -> &ColumnData {
&self.columns[26]
}
pub fn LeveVfx(&self) -> &ColumnData {
&self.columns[27]
}
pub fn LeveVfxFrame(&self) -> &ColumnData {
&self.columns[28]
}
pub fn CanCancel(&self) -> &ColumnData {
&self.columns[29]
}
pub fn LockedLeve(&self) -> &ColumnData {
&self.columns[30]
}
}
