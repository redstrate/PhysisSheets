#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringItem {
exd: EXD,
exh: EXH,
}
impl GatheringItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringItem").unwrap();let exd = game_data.read_excel_sheet("GatheringItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringItemRow { columns: row.columns.clone() }
}
}
pub struct GatheringItemRow {
columns: Vec<ColumnData>,
}
impl GatheringItemRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SublimeVariant(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[2]
}
pub fn GatheringItemLevel(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PerceptionReq(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[8]
}
pub fn IsHidden(&self) -> &ColumnData {
&self.columns[9]
}
}
