#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringItemLevelConvertTable {
exd: EXD,
exh: EXH,
}
impl GatheringItemLevelConvertTable {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringItemLevelConvertTable").unwrap();let exd = game_data.read_excel_sheet("GatheringItemLevelConvertTable", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringItemLevelConvertTableRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringItemLevelConvertTableRow { columns: row.columns.clone() }
}
}
pub struct GatheringItemLevelConvertTableRow {
columns: Vec<ColumnData>,
}
impl GatheringItemLevelConvertTableRow {
pub fn GatheringItemLevel(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[1]
}
}
