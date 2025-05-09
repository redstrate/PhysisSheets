#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SpearfishingSilhouette {
exd: EXD,
exh: EXH,
}
impl SpearfishingSilhouette {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SpearfishingSilhouette").unwrap();let exd = game_data.read_excel_sheet("SpearfishingSilhouette", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SpearfishingSilhouetteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SpearfishingSilhouetteRow { columns: row.columns.clone() }
}
}
pub struct SpearfishingSilhouetteRow {
columns: Vec<ColumnData>,
}
impl SpearfishingSilhouetteRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
