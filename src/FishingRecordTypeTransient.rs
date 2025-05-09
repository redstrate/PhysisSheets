#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FishingRecordTypeTransient {
exd: EXD,
exh: EXH,
}
impl FishingRecordTypeTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FishingRecordTypeTransient").unwrap();let exd = game_data.read_excel_sheet("FishingRecordTypeTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FishingRecordTypeTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FishingRecordTypeTransientRow { columns: row.columns.clone() }
}
}
pub struct FishingRecordTypeTransientRow {
columns: Vec<ColumnData>,
}
impl FishingRecordTypeTransientRow {
pub fn Image(&self) -> &ColumnData {
&self.columns[0]
}
}
