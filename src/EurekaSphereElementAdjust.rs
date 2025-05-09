#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaSphereElementAdjust {
exd: EXD,
exh: EXH,
}
impl EurekaSphereElementAdjust {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaSphereElementAdjust").unwrap();let exd = game_data.read_excel_sheet("EurekaSphereElementAdjust", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaSphereElementAdjustRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaSphereElementAdjustRow { columns: row.columns.clone() }
}
}
pub struct EurekaSphereElementAdjustRow {
columns: Vec<ColumnData>,
}
impl EurekaSphereElementAdjustRow {
pub fn PowerModifier(&self) -> &ColumnData {
&self.columns[0]
}
}
