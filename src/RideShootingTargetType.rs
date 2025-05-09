#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RideShootingTargetType {
exd: EXD,
exh: EXH,
}
impl RideShootingTargetType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RideShootingTargetType").unwrap();let exd = game_data.read_excel_sheet("RideShootingTargetType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RideShootingTargetTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RideShootingTargetTypeRow { columns: row.columns.clone() }
}
}
pub struct RideShootingTargetTypeRow {
columns: Vec<ColumnData>,
}
impl RideShootingTargetTypeRow {
pub fn EObj(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Score(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[5]
}
}
