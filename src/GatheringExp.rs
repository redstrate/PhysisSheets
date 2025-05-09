#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringExp {
exd: EXD,
exh: EXH,
}
impl GatheringExp {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringExp").unwrap();let exd = game_data.read_excel_sheet("GatheringExp", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringExpRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringExpRow { columns: row.columns.clone() }
}
}
pub struct GatheringExpRow {
columns: Vec<ColumnData>,
}
impl GatheringExpRow {
pub fn Exp(&self) -> &ColumnData {
&self.columns[0]
}
}
