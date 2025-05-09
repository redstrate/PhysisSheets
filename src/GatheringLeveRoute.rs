#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GatheringLeveRoute {
exd: EXD,
exh: EXH,
}
impl GatheringLeveRoute {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GatheringLeveRoute").unwrap();let exd = game_data.read_excel_sheet("GatheringLeveRoute", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GatheringLeveRouteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GatheringLeveRouteRow { columns: row.columns.clone() }
}
}
pub struct GatheringLeveRouteRow {
columns: Vec<ColumnData>,
}
impl GatheringLeveRouteRow {
pub fn GatheringPoint(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PopRange(&self) -> &ColumnData {
&self.columns[1]
}
}
