#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GCSupplyDuty {
exd: EXD,
exh: EXH,
}
impl GCSupplyDuty {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GCSupplyDuty").unwrap();let exd = game_data.read_excel_sheet("GCSupplyDuty", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GCSupplyDutyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GCSupplyDutyRow { columns: row.columns.clone() }
}
}
pub struct GCSupplyDutyRow {
columns: Vec<ColumnData>,
}
impl GCSupplyDutyRow {
pub fn SupplyData(&self) -> &ColumnData {
&self.columns[0]
}
}
