#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingMateAuthority {
exd: EXD,
exh: EXH,
}
impl HousingMateAuthority {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingMateAuthority").unwrap();let exd = game_data.read_excel_sheet("HousingMateAuthority", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingMateAuthorityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingMateAuthorityRow { columns: row.columns.clone() }
}
}
pub struct HousingMateAuthorityRow {
columns: Vec<ColumnData>,
}
impl HousingMateAuthorityRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
