#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FishingRecordType {
exd: EXD,
exh: EXH,
}
impl FishingRecordType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FishingRecordType").unwrap();let exd = game_data.read_excel_sheet("FishingRecordType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FishingRecordTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FishingRecordTypeRow { columns: row.columns.clone() }
}
}
pub struct FishingRecordTypeRow {
columns: Vec<ColumnData>,
}
impl FishingRecordTypeRow {
pub fn Addon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RankBRequirement(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RankARequirement(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RankAARequirement(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RankAAARequirement(&self) -> &ColumnData {
&self.columns[4]
}
pub fn RankSRequirement(&self) -> &ColumnData {
&self.columns[5]
}
pub fn IsSpearfishing(&self) -> &ColumnData {
&self.columns[6]
}
}
