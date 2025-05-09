#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BaseParam {
exd: EXD,
exh: EXH,
}
impl BaseParam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BaseParam").unwrap();let exd = game_data.read_excel_sheet("BaseParam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BaseParamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BaseParamRow { columns: row.columns.clone() }
}
}
pub struct BaseParamRow {
columns: Vec<ColumnData>,
}
impl BaseParamRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn OneHandWeaponPercent(&self) -> &ColumnData {
&self.columns[2]
}
pub fn OffHandPercent(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HeadPercent(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ChestPercent(&self) -> &ColumnData {
&self.columns[5]
}
pub fn HandsPercent(&self) -> &ColumnData {
&self.columns[6]
}
pub fn WaistPercent(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LegsPercent(&self) -> &ColumnData {
&self.columns[8]
}
pub fn FeetPercent(&self) -> &ColumnData {
&self.columns[9]
}
pub fn EarringPercent(&self) -> &ColumnData {
&self.columns[10]
}
pub fn NecklacePercent(&self) -> &ColumnData {
&self.columns[11]
}
pub fn BraceletPercent(&self) -> &ColumnData {
&self.columns[12]
}
pub fn RingPercent(&self) -> &ColumnData {
&self.columns[13]
}
pub fn TwoHandWeaponPercent(&self) -> &ColumnData {
&self.columns[14]
}
pub fn UnderArmorPercent(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ChestHeadPercent(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ChestHeadLegsFeetPercent(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[18]
}
pub fn LegsFeetPercent(&self) -> &ColumnData {
&self.columns[19]
}
pub fn HeadChestHandsLegsFeetPercent(&self) -> &ColumnData {
&self.columns[20]
}
pub fn ChestLegsGlovesPercent(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ChestLegsFeetPercent(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[23]
}
pub fn OrderPriority(&self) -> &ColumnData {
&self.columns[24]
}
pub fn MeldParam(&self) -> &ColumnData {
&self.columns[25]
}
pub fn PacketIndex(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[27]
}
}
