#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AOZReport {
exd: EXD,
exh: EXH,
}
impl AOZReport {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZReport").unwrap();let exd = game_data.read_excel_sheet("AOZReport", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZReportRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AOZReportRow { columns: row.columns.clone() }
}
}
pub struct AOZReportRow {
columns: Vec<ColumnData>,
}
impl AOZReportRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[2]
}
}
